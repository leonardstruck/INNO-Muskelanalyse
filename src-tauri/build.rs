fn main() {
    build_analysis_script();
    build_segmentation_script();

    tauri_build::build()
}

fn build_segmentation_script() {
    use cmake::Config;

    let segmentation_path = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join("segmentation");
    let segmentation_bin = Config::new(segmentation_path).build();

    // move binary to target folder if build was not up to date
    if std::path::Path::new(&segmentation_bin).join("bin").exists() {
        move_binary(
            segmentation_bin.join("bin").to_str().unwrap(),
            "segmentation",
        );
    }
}

fn build_analysis_script() {
    use std::process::Command;

    let analysis_path = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join("analysis");

    // delete old bin folder
    std::fs::remove_dir_all(analysis_path.join("bin")).unwrap_or(());

    let mut command = Command::new("pyinstaller");
    command
        .arg("--onefile")
        .arg("--specpath")
        .arg(analysis_path.join("bin"))
        .arg("--distpath")
        .arg(analysis_path.join("bin"))
        .arg(analysis_path.join("analysis.py"));

    // wait for command to finish
    let output = command.output().unwrap();

    // check if command was successful
    if !output.status.success() {
        panic!(
            "Failed to build analysis script: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // print output of command
    cargo_emit::warning!("{}", String::from_utf8_lossy(&output.stdout));

    move_binary(analysis_path.join("bin").to_str().unwrap(), "analysis");
}

fn move_binary(out_dir: &str, target_name: &str) {
    use current_platform::CURRENT_PLATFORM;

    // check if platform is windows
    let mut extension = "";

    if cfg!(windows) {
        extension = ".exe";
    }

    let target_triple = CURRENT_PLATFORM;

    // generate target path

    let target_path = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join(target_name)
        .join("bin")
        .join(format!("{}-{}{}", target_name, target_triple, extension));

    // check if directory exists
    if !target_path.parent().unwrap().exists() {
        std::fs::create_dir_all(target_path.parent().unwrap()).unwrap();
    }

    // generate original path

    let original_path = std::path::Path::new(out_dir).join(format!("{}{}", target_name, extension));

    // delete target file if it exists
    std::fs::remove_file(&target_path).unwrap_or(());

    // copy binary to target path
    std::fs::copy(original_path, target_path).unwrap();
}
