fn main() {
    use cmake::Config;

    let segmentation_path = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join("segmentation");
    let segmentation_bin = Config::new(segmentation_path).build();

    move_binary(
        segmentation_bin.join("bin").to_str().unwrap(),
        "segmentation",
    );

    tauri_build::build()
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
