use tauri::api::process::Command;

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
        format!("./{}", name)
    };

    name
}

pub fn resolve_bin_path(app: &tauri::AppHandle, name: &str) -> String {
    let resource_dir = resolve_bin_dir(app);
    let name = resolve_bin_name(name);

    resource_dir.join(name).to_str().unwrap().to_string()
}

pub fn python_command(app: tauri::AppHandle, name: &str) -> Result<Command, String> {
    let resource_path = app
        .path_resolver()
        .resource_dir()
        .ok_or_else(|| "Failed to get resource dir".to_string())?;

    let resource_path = dunce::canonicalize(resource_path).map_err(|e| e.to_string())?;

    let vendor_dir = resource_path.join("vendor");
    let main_py_path = vendor_dir.join(name).join("main.py");

    // check if main.py exists
    if !main_py_path.exists() {
        return Err(format!(
            "Failed to find main.py for vendor: {}",
            main_py_path.to_str().unwrap()
        ));
    }

    // resolve venv path
    let venv_path = app
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("venv")
        .join(name);

    let venv_path = dunce::canonicalize(venv_path).map_err(|e| e.to_string())?;

    // check if venv exists
    if !venv_path.exists() {
        return Err(format!(
            "Failed to find venv for vendor: {}",
            venv_path.to_str().unwrap()
        ));
    }

    let python_path = match cfg!(target_os = "windows") {
        true => venv_path.join("Scripts").join("python.exe"),
        false => venv_path.join("bin").join("python"),
    };

    // check if python3 exists
    if !python_path.exists() {
        return Err(format!(
            "Failed to find python3 for vendor: {}",
            python_path.to_str().unwrap()
        ));
    }

    let python_path = match python_path.to_str() {
        Some(python_path) => python_path,
        None => {
            return Err(format!(
                "Failed to convert python path to string: {:?}",
                python_path
            ))
        }
    };

    let command = Command::new(python_path).args([main_py_path.to_str().unwrap()]);

    Ok(command)
}
