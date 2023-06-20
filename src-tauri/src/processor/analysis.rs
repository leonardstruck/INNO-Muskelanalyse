use log::debug;
use tauri::Manager;

use crate::{
    models::segments::{Segment, SegmentChangeset, Status},
    processor::ProcessorState,
    state::AppState,
    utils,
};

use serde::Deserialize;

use super::Processor;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum AnalysisStatus {
    Error,
    Success,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnalysisResult {
    direction_a: f32,
    direction_b: f32,
    angle: f32,
    midpoint_x: f32,
    midpoint_y: f32,
    status: AnalysisStatus,
}

#[derive(Deserialize, Debug)]
struct AnalysisOutput {
    status: String,
    data: Vec<AnalysisResult>,
}

impl Processor {
    pub fn run_analysis(&self) {
        let app = self.app.app_handle();
        let project_id = self.project_id.clone();
        let micrograph_id = self.micrograph_id.clone();

        tauri::async_runtime::spawn(async move {
            let state = app.state::<AppState>();

            // get all segments that are not yet analyzed
            let segments = match state.get_segments_by_micrograph(&project_id, &micrograph_id) {
                Ok(segments) => segments,
                Err(err) => {
                    debug!("Failed to get segments: {:?}", err);
                    return;
                }
            };

            // filter out segments that are already analyzed
            let segments: Vec<Segment> = segments
                .into_iter()
                .filter(|segment| segment.status != Status::Ok)
                .collect();

            debug!("Found {} segments to analyze", segments.len());

            // update total cound of jobs in processor state (wrapped in mutex)
            {
                let processor_state = app.state::<ProcessorState>();
                let mut processor_state = processor_state
                    .0
                    .get_mut(&micrograph_id.to_string())
                    .unwrap();
                processor_state.total_jobs = Some(segments.len());
                processor_state.completed_jobs = Some(0);
            }

            // split segments into chunks of 50
            let segment_chunks: Vec<Vec<Segment>> = segments.chunks(50).map(|x| x.to_vec()).collect();

            // iterate over segments and run analysis
            for segment_chunk in segment_chunks {
                debug!("Analyzing segments: {:?}", segment_chunk.iter().map(|x| x.uuid.clone()).collect::<Vec<String>>());
                let mut command = match utils::python_command(app.app_handle(), "analysis") {
                    Ok(command) => command,
                    Err(err) => {
                        debug!("Failed to resolve python command: {:?}", err);
                        return;
                    }
                };

                let cached_segments = segment_chunk
                    .iter()
                    .map(|segment| segment.to_cache(&app.app_handle()))
                    .collect::<Vec<_>>();

                let image_paths = cached_segments
                    .iter()
                    .map(|segment| segment.binary_img.to_str().unwrap())
                    .collect::<Vec<_>>();

                command = command.args(&image_paths);

                let output = match command.output() {
                    Ok(output) => output,
                    Err(err) => {
                        debug!("Failed to run python command: {:?}", err);
                        return;
                    }
                };

                // parse output
                let analysis_output: AnalysisOutput = match serde_json::from_str(&output.stdout) {
                    Ok(output) => output,
                    Err(err) => {
                        debug!("Failed to parse python output: {:?}", err);
                        debug!("Python output: {:?}", output.stdout);
                        debug!("Python error: {:?}", output.stderr);
                        return;
                    }
                };

                // store analysis results in database
                let analysis_results = analysis_output.data;

                for (index, analysis_result) in analysis_results.iter().enumerate() {
                    let changeset = SegmentChangeset {
                        uuid: cached_segments[index].uuid.clone(),
                        measured_angle: Some(analysis_result.angle),
                        measured_length: Some(analysis_result.direction_a),
                        measured_width: Some(analysis_result.direction_b),
                        measured_midpoint_x: Some(analysis_result.midpoint_x),
                        measured_midpoint_y: Some(analysis_result.midpoint_y),
                        height: None,
                        width: None,
                        location_x: None,
                        location_y: None,
                        status: Some(Status::Ok),
                    };
    
                    match state.update_segment(&project_id, &changeset) {
                        Err(err) => debug!("Failed to update segment: {:?}", err),
                        _ => (),
                    }
    
                    // update job count in processor state (wrapped in mutex)
                    {
                        let processor_state = app.state::<ProcessorState>();
                        let mut processor_state = processor_state
                            .0
                            .get_mut(&micrograph_id.to_string())
                            .unwrap();
                        processor_state.completed_jobs = processor_state.completed_jobs.map(|x| x + 1);
                    }
                }    
            }

            // update processor state
            {
                let processor_state = app.state::<ProcessorState>();
                let mut processor_state = processor_state
                    .0
                    .get_mut(&micrograph_id.to_string())
                    .unwrap();
                processor_state.total_jobs = None;
                processor_state.completed_jobs = None;
            }

            // update micrograph status
            match state.update_micrograph_status(
                &project_id,
                &micrograph_id,
                crate::models::micrographs::Status::Done,
            ) {
                Ok(_) => {
                    debug!("Successfully updated micrograph status");
                }
                Err(err) => {
                    debug!("Failed to update micrograph status: {:?}", err);
                }
            }

            // send event to frontend
            let _ = app.emit_to(&project_id.to_string(), "UPDATE_MICROGRAPHS", ());
        });
    }
}
