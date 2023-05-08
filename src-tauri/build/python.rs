pub struct Builder {
    vendor_dir: std::path::PathBuf,
    paths: Vec<std::path::PathBuf>,
    out_dir: std::path::PathBuf,
}

impl Builder {
    pub fn new() -> Self {
        let vendor_dir = crate::utils::get_vendor_dir();

        Self {
            vendor_dir,
            paths: Vec::new(),
            out_dir: crate::utils::get_output_dir(),
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

        let name = path.file_name().unwrap().to_str().unwrap();

        // check if binary already exists
        let bin_dir = crate::utils::get_bin_dir();
        if bin_dir.join(name).exists() {
            // check if binary is up to date
            let bin_metadata = std::fs::metadata(bin_dir.join(name)).unwrap();

            // iterate over all files in vendor directory
            for entry in std::fs::read_dir(path.clone()).unwrap() {
                let entry = entry.unwrap();
                let file_path = entry.path();

                // check if path is a file
                if file_path.is_file() {
                    // check if file is newer than binary
                    let metadata = std::fs::metadata(file_path.clone()).unwrap();
                    if metadata.modified().unwrap() > bin_metadata.modified().unwrap() {
                        // rebuild binary
                        self.paths.push(path);
                        return;
                    }
                }
            }
        } else {
            // build binary
            self.paths.push(path);
        }
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
        self.copy_files();
    }

    fn copy_files(&self) {
        let resource_dir = crate::utils::get_bin_dir();

        // copy all folders and files recursively to the target directory
        let folders =
            std::fs::read_dir(self.out_dir.clone().join("python").join("main.dist")).unwrap();

        for folder in folders {
            let folder = folder.unwrap();
            let path = folder.path();

            // check if path is a file
            if path.is_file() {
                // copy file
                fs_extra::file::copy(
                    path.clone(),
                    resource_dir.clone().join(path.file_name().unwrap()),
                    &fs_extra::file::CopyOptions::new().overwrite(true),
                )
                .unwrap();
                continue;
            }

            // copy folder recursively
            fs_extra::dir::copy(
                path,
                resource_dir.clone(),
                &fs_extra::dir::CopyOptions::new().overwrite(true),
            )
            .unwrap();
        }
    }
}

fn build_python_projects(paths: Vec<std::path::PathBuf>) {
    use std::process::Command;

    let target_dir = crate::utils::get_output_dir();

    // create target directory if it doesn't exist
    std::fs::create_dir_all(target_dir.clone()).unwrap();

    // build python projects

    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        let output = Command::new("python")
            .arg("-m")
            .arg("nuitka")
            .arg("--static-libpython=no")
            .arg("--standalone")
            .arg(format!("--output-filename={}", name))
            .arg(format!(
                "--output-dir={}",
                target_dir.join("python").to_str().unwrap()
            ))
            .arg("main.py")
            .current_dir(path)
            .output();

        let output = match output {
            Ok(output) => output,
            Err(err) => panic!("Failed to build python project: {}", err),
        };

        // check if command was successful
        if !output.status.success() {
            panic!(
                "Failed to build python project: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }
}
