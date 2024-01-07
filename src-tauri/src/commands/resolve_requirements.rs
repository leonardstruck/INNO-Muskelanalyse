use std::{path::PathBuf, process::exit};

use log::{debug, error};
use tauri::{
    api::{dialog::blocking::message, process::Command},
    Manager,
};

#[tauri::command]
pub async fn check_requirements(
    app: tauri::AppHandle,
    window: tauri::Window,
) -> Result<bool, String> {
    // check if python is installed
    match resolve_python_bin() {
        Ok(_) => {}
        Err(e) => {
            error!("Error resolving python: {}", e);
            let _ = message(
                Some(&window),
                "Python not found",
                "Python is required to run this application. Please install Python and try again.",
            );
            exit(1);
        }
    }

    let vendors = resolve_python_vendors(app.app_handle())?;

    // ensure virtual environments exist for each vendor
    for vendor in &vendors {
        ensure_python_venv(app.app_handle(), vendor.to_path_buf())?;
    }

    // install requirements for each vendor
    for vendor in &vendors {
        match install_dependencies(app.app_handle(), vendor.to_path_buf()) {
            Ok(_) => {}
            Err(e) => {
                error!("Error installing dependencies: {}", e);
                return Err(format!(
                    "Error installing dependencies: {}, {:?}",
                    e, vendor
                ));
            }
        }
    }

    Ok(true)
}

#[memoize::memoize]
fn resolve_python_bin() -> Result<String, String> {
    // first check if python is set in the environment variables
    if let Ok(python_path) = std::env::var("PYTHON_BIN") {
        if std::path::Path::new(&python_path).exists() {
            debug!("Found python in environment variables: {}", python_path);
            return Ok(python_path);
        }
    }

    // if not, check if python3 is in the path
    if let Ok(python_path) = which::which("python3") {
        if python_path.exists() {
            debug!("Found python3 in path: {}", python_path.to_string_lossy());
            return Ok(python_path.to_str().unwrap().to_string());
        }
    }

    // if not, check if python is in the path
    if let Ok(python_path) = which::which("python") {
        if python_path.exists() {
            debug!("Found python in path: {}", python_path.to_string_lossy());
            return Ok(python_path.to_str().unwrap().to_string());
        }
    }

    return Err("Python not found".to_string());
}

fn get_vendor_dir(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_path = app
        .path_resolver()
        .resource_dir()
        .ok_or_else(|| "Failed to get resource dir".to_string())?;

    let vendor_dir = resource_path.join("vendor");
    let vendor_dir = dunce::canonicalize(vendor_dir).map_err(|e| e.to_string())?;

    Ok(vendor_dir)
}

fn resolve_python_vendors(app: tauri::AppHandle) -> Result<Vec<PathBuf>, String> {
    let mut vendors = Vec::new();

    let vendor_dir = get_vendor_dir(app)?;

    // iterate through vendor directory and check each folder for a requirements.txt file
    // errors have to be converted to strings because of the ? operator
    for entry in std::fs::read_dir(vendor_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_dir() {
            let requirements_path = path.join("requirements.txt");

            if requirements_path.exists() {
                vendors.push(path);
            }
        }
    }

    Ok(vendors)
}

fn ensure_python_venv(app: tauri::AppHandle, path: PathBuf) -> Result<(), String> {
    let python_bin = resolve_python_bin()?;

    let venv_name = path.file_name().unwrap().to_str().unwrap();
    let venv_path = app
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("venv")
        .join(venv_name);

    if !venv_path.exists() {
        let output = Command::new(python_bin)
            .args(["-m", "venv", venv_path.to_str().unwrap()])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!(
                "Failed to create virtual environment: {}",
                output.stderr
            ));
        } else {
            debug!(
                "Created virtual environment: {:?}, {}",
                venv_path, output.stdout
            );
        }
    } else {
        debug!("Virtual environment already exists: {:?}", venv_path);
    }

    Ok(())
}

fn install_dependencies(app: tauri::AppHandle, path: PathBuf) -> Result<(), String> {
    let venv_name = path.file_name().unwrap().to_str().unwrap();
    let venv_path = app
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("venv")
        .join(venv_name);

    let requirements_path = path.join("requirements.txt");
    let requirements_path = dunce::canonicalize(requirements_path).map_err(|e| e.to_string())?;

    debug!(
        "Installing dependencies from {:?} in {:?}",
        requirements_path, venv_path
    );

    let pip_path = match std::env::consts::OS {
        "windows" => venv_path.join("Scripts").join("pip.exe"),
        _ => venv_path.join("bin").join("pip"),
    };

    let pip_path = dunce::canonicalize(pip_path).map_err(|e| e.to_string())?;

    // install dependencies inside virtual environment
    let output = Command::new(pip_path.to_str().unwrap())
        .args(["install", "-r", requirements_path.to_str().unwrap()])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!("Failed to install dependencies: {}", output.stderr));
    } else {
        debug!("Installed dependencies: {:?}, {}", venv_path, output.stdout);
    }

    Ok(())
}
