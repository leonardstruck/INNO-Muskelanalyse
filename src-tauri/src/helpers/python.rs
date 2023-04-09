pub struct Command;

impl Command {
    pub fn new(name: &str, app: &tauri::AppHandle) -> tauri::api::process::Command {
        // check if platform is windows
        let mut extension = "";

        if cfg!(windows) {
            extension = ".exe";
        }

        let python_resources = app
            .path_resolver()
            .resource_dir()
            .unwrap()
            .join("build/python/dist");

        let program = format!(
            "{python_resources}/{name}{extension}",
            python_resources = python_resources.to_str().unwrap(),
            name = name,
            extension = extension
        );

        tauri::api::process::Command::new(program).current_dir(python_resources)
    }
}
