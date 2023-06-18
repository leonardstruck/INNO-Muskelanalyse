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

pub struct FileAssociation {
    pub name: String,
    pub extensions: Vec<String>,
}

pub fn resolve_file_association() -> Result<FileAssociation, ()> {
    // open package.json
    let package_json = std::fs::read_to_string("../package.json").unwrap();

    // parse package.json
    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();

    // get filetypeAssociation object
    let file_association = package_json["filetypeAssociation"].as_object().unwrap();

    // get filetypeAssociation.name
    let name = file_association["name"].as_str().unwrap();

    // get filetypeAssociation.extensions
    let extensions = file_association["extensions"].as_array().unwrap();

    Ok(FileAssociation {
        name: name.to_string(),
        extensions: extensions
            .iter()
            .map(|extension| extension.as_str().unwrap().to_string())
            .collect(),
    })
}

pub fn python_command(app: tauri::AppHandle, name: &str) -> Result<Command, String> {
    let resource_path = app
        .path_resolver()
        .resource_dir()
        .ok_or_else(|| "Failed to get resource dir".to_string())?;

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

    // check if venv exists
    if !venv_path.exists() {
        return Err(format!(
            "Failed to find venv for vendor: {}",
            venv_path.to_str().unwrap()
        ));
    }

    let command = Command::new("python3")
        .args([main_py_path.to_str().unwrap()])
        .current_dir(venv_path.join("bin"));

    Ok(command)
}
