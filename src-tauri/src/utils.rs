pub fn resolve_bin_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    let resource_dir = app.path_resolver().resource_dir().unwrap();

    let resource_dir = resource_dir.join("target").join("bin");
    let resource_dir = resource_dir.strip_prefix("\\\\?\\").unwrap();

    resource_dir.to_path_buf()
}

pub fn resolve_bin_name(name: &str) -> String {
    let name = if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };

    format!("./{}", name)
}
