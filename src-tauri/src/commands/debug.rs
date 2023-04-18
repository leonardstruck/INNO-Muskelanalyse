#[tauri::command]
pub async fn debug_segmentation(app: tauri::AppHandle) -> Result<(), String> {
    // run segmentation

    let command = tauri::api::process::Command::new(crate::utils::resolve_bin_path(&app, "segmentation"))
        .current_dir(crate::utils::resolve_bin_dir(&app));

    // print command to console
    println!("Running command: {:?}", command);

    let output = command.output().unwrap();

    println!("stdout: {}", output.stdout);
    println!("stderr: {}", output.stderr);

    Ok(())
}