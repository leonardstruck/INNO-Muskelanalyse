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
