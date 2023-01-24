fn main() {
    use cmake::Config;
    let segmentation_bin = Config::new("../resources/segmentation").build();

    move_binary(
        segmentation_bin.join("bin/segmentation").to_str().unwrap(),
        "segmentation",
    );

    tauri_build::build()
}

fn move_binary(original_path: &str, target_name: &str) {
    use current_platform::CURRENT_PLATFORM;

    // check if platform is windows
    let mut extension = "";
    if cfg!(windows) {
        extension = ".exe";
    }

    let target_triple = CURRENT_PLATFORM;

    // generate target path
    let target_path = format!("binaries/{}-{}{}", target_name, target_triple, extension);
    let target_path = std::env::current_dir().unwrap().join(target_path);

    // check if directory exists
    if !target_path.parent().unwrap().exists() {
        std::fs::create_dir_all(target_path.parent().unwrap()).unwrap();
    }

    // copy binary to target path
    std::fs::copy(original_path, target_path).unwrap();
}
