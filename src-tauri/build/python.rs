use crate::utils::get_bin_dir;

pub struct Builder {
    vendor_dir: std::path::PathBuf,
    paths: Vec<std::path::PathBuf>,
    libs: Vec<std::path::PathBuf>,
    out_dir: std::path::PathBuf,
}

impl Builder {
    pub fn new() -> Self {
        let vendor_dir = crate::utils::get_vendor_dir();

        Self {
            vendor_dir,
            paths: Vec::new(),
            out_dir: crate::utils::get_output_dir(),
            libs: Vec::new(),
        }
    }

    pub fn add_vendor(&mut self, name: &str) {
        let path = self.vendor_dir.join(name);

        // check if path exists
        if !path.exists() {
            panic!("vendor directory does not exist");
        }

        // tell cargo to rerun build script if vendor directory changes
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

        self.paths.push(path);
    }

    fn resolve_requirements(&self) {
        for path in self.paths.clone() {
            // check if requirements.txt is present and install modules
            let requirements_path = path.join("requirements.txt");
            if requirements_path.exists() {
                use std::process::Command;

                let output = Command::new("python")
                    .arg("-m")
                    .arg("pip")
                    .arg("install")
                    .arg("-r")
                    .arg(requirements_path)
                    .output()
                    .unwrap();

                // check if command was successful
                if !output.status.success() {
                    panic!(
                        "Failed to install requirements: {}",
                        String::from_utf8(output.stderr).unwrap()
                    );
                }
            }
        }
    }

    pub fn build(&mut self) {
        self.resolve_requirements();
        build_python_projects(self.paths.clone());

        // resolve libs
        self.libs = resolve_libs(self.out_dir.clone().join("python"));

        self.copy_files();
    }

    fn copy_files(&self) {
        // copy all libs to the target directory
        let resource_dir = crate::utils::get_bin_dir();

        for lib in self.libs.clone() {
            let target_path = resource_dir.join(lib.file_name().unwrap());

            // copy file only if it doesn't exist
            if !target_path.exists() {
                std::fs::copy(lib, target_path).unwrap();
            }
        }

        // copy all generated .zip files to the target directory
        let files = std::fs::read_dir(self.out_dir.clone().join("python")).unwrap();

        for file in files {
            let file = file.unwrap();
            let path = file.path();

            // skip directories
            if path.is_dir() {
                continue;
            }

            // check if file has a file extension
            if path.extension().is_none() {
                continue;
            }

            // check if file is a zip file
            if path.extension().unwrap() == "zip" {
                // copy file only if it doesn't exist
                let target_path = resource_dir.join(path.file_name().unwrap());
                if !target_path.exists() {
                    std::fs::copy(path, target_path).unwrap();
                }
            }
        }

        // copy all folders recursively to the target directory
        let folders = std::fs::read_dir(self.out_dir.clone().join("python")).unwrap();

        for folder in folders {
            let folder = folder.unwrap();
            let path = folder.path();

            // skip files
            if path.is_file() {
                continue;
            }

            // copy folder recursively
            fs_extra::dir::copy(
                path,
                resource_dir.clone(),
                &fs_extra::dir::CopyOptions::new().skip_exist(true),
            )
            .unwrap();
        }

        // copy all binaries to the target directory
        for path in &self.paths {
            let target_path = get_bin_dir().join(path.file_name().unwrap());

            let bin_path = self
                .out_dir
                .clone()
                .join("python")
                .join(path.file_name().unwrap());

            // copy file only if it doesn't exist or if it is newer
            if !target_path.exists()
                || target_path.metadata().unwrap().modified().unwrap()
                    < bin_path.metadata().unwrap().modified().unwrap()
            {
                std::fs::copy(bin_path, target_path).unwrap();
            }
        }
    }
}

fn build_python_projects(paths: Vec<std::path::PathBuf>) {
    use std::process::Command;

    let target_dir = crate::utils::get_output_dir();

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
    name='python'
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
        .arg("--workpath")
        .arg(target_dir.clone().join("build"))
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

fn resolve_libs(out_path: std::path::PathBuf) -> Vec<std::path::PathBuf> {
    // scan build directory for libraries (dlls, so, dylib)
    let libs = std::fs::read_dir(out_path).unwrap();

    // create a vector of all paths
    let mut paths: Vec<std::path::PathBuf> = Vec::new();

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
                paths.push(path.clone());
            }
        }
    }

    paths
}
