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
