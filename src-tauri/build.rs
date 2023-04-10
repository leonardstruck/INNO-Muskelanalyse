fn main() {
    resolve_dependencies();

    let cpp_projects = resolve_cpp();
    build_cpp_projects(cpp_projects);

    let python_projects = resolve_python();
    build_python_projects(python_projects);

    tauri_build::build()
}

//
// CPP
//

fn resolve_cpp() -> Vec<std::path::PathBuf> {
    let path = get_resource_dir().join("cpp");

    // check if directory exists
    if !path.exists() {
        panic!("cpp directory does not exist");
    }

    // find all subdirectories
    let subdirs = std::fs::read_dir(path).unwrap();

    // create a vector of all paths
    let mut paths = Vec::new();

    for subdir in subdirs {
        let subdir = subdir.unwrap();
        let path = subdir.path();

        // check if path is a directory
        if path.is_dir() {
            paths.push(path);
        }
    }

    paths
}

fn build_cpp(path: std::path::PathBuf) {
    use cmake::Config;

    let path_binding = path.clone();
    let stripped_path = path_binding
        .clone()
        .to_str()
        .unwrap()
        .replace("\\\\?\\", "");
    let target_name = path_binding.file_name().unwrap().to_str().unwrap();
    let extension = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    let bin = Config::new(stripped_path).build();
    let bin_parent = bin.parent().unwrap();

    cargo_emit::rustc_link_search!(bin_parent.display() => "native");

    // scan build directory for libraries (dlls, so, dylib)
    let libs = std::fs::read_dir(bin_parent).unwrap();

    for lib in libs {
        let lib = lib.unwrap();
        let path = lib.path();

        // check if path is a file
        if path.is_file() {
            // check if file is a library
            if path.extension().unwrap_or_default() == "dll"
                || path.extension().unwrap_or_default() == "so"
                || path.extension().unwrap_or_default() == "dylib"
            {
                cargo_emit::rerun_if_changed!(path.display());
                cargo_emit::rustc_link_lib!(path.file_stem().unwrap().to_str().unwrap() => "dylib");
            }
        }
    }

    // move binary to target folder if build was not up to date
    if std::path::Path::new(&bin)
        .join("bin")
        .join(target_name.to_string() + extension)
        .exists()
    {
        // create target directory if it doesn't exist
        let target_dir = std::env::current_dir().unwrap().join("build/cpp");
        std::fs::create_dir_all(target_dir.clone()).unwrap();

        let target_dir_name = target_dir.join(append_target_triple(target_name));
        // move binary to target directory
        std::fs::rename(
            bin.join("bin").join(target_name.to_string() + extension),
            target_dir_name.clone(),
        )
        .expect("failed to move binary");

        cargo_emit::rustc_link_search!(target_dir_name.to_str().unwrap() => "native");
        cargo_emit::rustc_link_lib!(target_name => "static");
    }
}

fn build_cpp_projects(paths: Vec<std::path::PathBuf>) {
    for path in paths {
        build_cpp(path);
    }
}

//
// PYTHON
//

fn resolve_python() -> Vec<std::path::PathBuf> {
    let path = get_resource_dir().join("python");

    // check if directory exists
    if !path.exists() {
        panic!("python directory does not exist");
    }

    // find all subdirectories
    let subdirs = std::fs::read_dir(path).unwrap();

    // create a vector of all paths
    let mut paths = Vec::new();

    for subdir in subdirs {
        let subdir = subdir.unwrap();
        let path = subdir.path();

        cargo_emit::rerun_if_changed!(format!("{}/*.py", path.to_str().unwrap()));

        // check if path is a directory
        if path.is_dir() {
            paths.push(path.clone());

            // check if requirements.txt is present and install modules
            let requirements_path = path.join("requirements.txt");
            if requirements_path.exists() {
                use std::process::Command;

                let output = Command::new("pip")
                    .arg("install")
                    .arg("-r")
                    .arg(requirements_path)
                    .output()
                    .expect("failed to execute process");

                // panic if pip failed
                if !output.status.success() {
                    panic!(
                        "pip failed with exit code: {}",
                        output.status.code().unwrap()
                    );
                }
            }
        }
    }

    paths
}

fn build_python_projects(paths: Vec<std::path::PathBuf>) {
    use std::process::Command;

    let target_dir = std::env::current_dir().unwrap().join("build/python");

    // create target directory if it doesn't exist
    std::fs::create_dir_all(target_dir.clone()).unwrap();

    // create new spec file
    let spec_file = target_dir.join("python.spec");

    // delete old spec file
    std::fs::remove_file(spec_file.clone()).unwrap_or(());

    // create new spec file
    std::fs::File::create(spec_file.clone()).unwrap();

    let mut spec_content = String::new();

    // append spec head to spec_content
    spec_content.push_str(
        r#"# -*- mode: python ; coding: utf-8 -*-
    
block_cipher = None
"#,
    );
    spec_content.push_str("\n\n");

    // create analysis script
    for path in paths.clone() {
        let path_binding = path.clone();
        let target_name = path_binding.file_name().unwrap().to_str().unwrap();

        // append to spec file
        spec_content.push_str(
            format!(
                r#"{} = Analysis(['{}'],
                pathex=['{}'],
                binaries=[],
                datas=[],
                hiddenimports=[],
                hookspath=[],
                hooksconfig=[],
                runtime_hooks=[],
                excludes=[],
                win_no_prefer_redirects=False,
                win_private_assemblies=False,
                cipher=block_cipher,
                noarchive=False
                )
            "#,
                target_name,
                path.join("main.py").to_str().unwrap().replace("\\", "/"),
                path.to_str().unwrap().replace("\\", "/")
            )
            .as_str(),
        );
    }

    spec_content.push('\n');

    // merge scripts
    spec_content.push_str("MERGE ( ");
    for path in paths.clone() {
        let path_binding = path.clone();
        let target_name = path_binding.file_name().unwrap().to_str().unwrap();

        // append to spec file
        spec_content
            .push_str(format!(r#"( {target_name}, '{target_name}', '{target_name}' )"#).as_str());
        // append comma if not last element
        if path != *paths.last().unwrap() {
            spec_content.push_str(", ");
        }
    }
    spec_content.push_str(" )\n\n");

    // create pyz script
    for path in paths.clone() {
        let path_binding = path.clone();
        let target_name = path_binding.file_name().unwrap().to_str().unwrap();

        // append to spec file
        spec_content.push_str(
            format!(
                r#"{target_name}_pyz = PYZ({target_name}.pure)
            "#
            )
            .as_str(),
        );
    }

    // create executables
    for path in paths.clone() {
        let path_binding = path.clone();
        let target_name = path_binding.file_name().unwrap().to_str().unwrap();

        // append to spec file
        spec_content.push_str(
            format!(
                r#"
{target_name}_exe = EXE(
    {target_name}_pyz,
    {target_name}.scripts,
    [],
    exclude_binaries=True,
    name='{target_name}',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
            "#
            )
            .as_str(),
        );
    }

    // collect
    spec_content.push_str(
        format!(
            r#"
coll = COLLECT("#
        )
        .as_str(),
    );
    for path in paths.clone() {
        let path_binding = path.clone();
        let target_name = path_binding.file_name().unwrap().to_str().unwrap();

        // append to spec file
        spec_content.push_str(
            format!(
                r#"
    {target_name}_exe,
    {target_name}.binaries,
    {target_name}.zipfiles,
    {target_name}.datas,"#,
            )
            .as_str(),
        );

        // append comma if not last element
        if path != *paths.last().unwrap() {
            spec_content.push_str(", ");
        }
    }

    spec_content.push_str(
        format!(
            r#"
    strip=False,
    upx=True,
    upx_exclude=[],
    name='dist'
)
            "#
        )
        .as_str(),
    );

    // write out spec_content to spec_file
    std::fs::write(spec_file.clone(), spec_content).unwrap();

    // build python projects
    let mut command = Command::new("python");
    command
        .arg("-m")
        .arg("PyInstaller")
        .arg("--distpath")
        .arg(target_dir.clone())
        .arg("-y")
        .arg(spec_file);

    // wait for command to finish
    let output = command.output().unwrap();

    // check if command was successful
    if !output.status.success() {
        panic!(
            "Failed to build python projects: {}",
            String::from_utf8(output.stderr).unwrap()
        );
    }
}

//
// UTILS
//

fn append_target_triple(target_name: &str) -> String {
    use current_platform::CURRENT_PLATFORM;

    // check if platform is windows
    let mut extension = "";

    if cfg!(windows) {
        extension = ".exe";
    }

    let target_triple = CURRENT_PLATFORM;

    format!("{}-{}{}", target_name, target_triple, extension)
}

fn resolve_dependencies() {
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

fn get_resource_dir() -> std::path::PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("..")
        .join("resources")
        .canonicalize()
        .expect("failed to resolve resource directory")
}
