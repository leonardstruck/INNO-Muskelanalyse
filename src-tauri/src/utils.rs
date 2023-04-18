pub fn resolve_bin_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    let resource_dir = app.path_resolver().resource_dir().unwrap();

    let resource_dir = resource_dir.join("target").join("bin");
    
    // replace long path with ""

    let resource_dir = resource_dir.to_str().unwrap().replace("\\\\?\\", "");

    resource_dir.into()
}

pub fn resolve_bin_name(name: &str) -> String {
    let name = if cfg!(target_os = "windows") {
        format!(".\\{}.exe", name)
    } else {
        format!("./{}", name.to_string())
    };

    name
}

pub fn resolve_bin_path(app: &tauri::AppHandle, name: &str) -> String {
    let resource_dir = resolve_bin_dir(app);
    let name = resolve_bin_name(name);

    resource_dir.join(name).to_str().unwrap().to_string()
}
