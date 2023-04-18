pub fn resolve_dependencies() {
    // check if python package pyinstaller is installed
    let output = std::process::Command::new("python")
        .arg("-m")
        .arg("PyInstaller")
        .arg("--version")
        .output();

    // check if command was successful
    if !output.is_ok() {
        cargo_emit::warning!("pyinstaller not found, installing it now");

        // install pyinstaller
        let output = std::process::Command::new("python")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("pyinstaller")
            .output()
            .unwrap();

        // check if command was successful
        if !output.status.success() {
            panic!(
                "Failed to install pyinstaller: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }
}

pub fn get_vendor_dir() -> std::path::PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("vendor")
        .canonicalize()
        .expect("failed to resolve vendor directory")
}

pub fn get_bin_dir() -> std::path::PathBuf {
    let bin_dir = std::env::current_dir().unwrap().join("target").join("bin");

    if !bin_dir.exists() {
        std::fs::create_dir_all(bin_dir.clone()).unwrap();
    }

    bin_dir
}

pub fn get_output_dir() -> std::path::PathBuf {
    std::env::var("OUT_DIR")
        .unwrap()
        .parse()
        .expect("failed to resolve output directory")
}

pub fn clear_bin_dir() {
    // check if target is release or debug

    let target = std::env::var("PROFILE").unwrap();

    let bin_dir = std::env::current_dir()
        .unwrap()
        .join("target")
        .join(target)
        .join("target");

    if bin_dir.exists() {
        std::fs::remove_dir_all(bin_dir).unwrap();
    }
}
