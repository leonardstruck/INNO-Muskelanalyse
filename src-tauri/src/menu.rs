use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

#[derive(Clone, serde::Serialize)]
struct Payload {}

pub fn create_menu() -> Menu {
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "App",
            Menu::new().add_item(CustomMenuItem::new("quit", "Quit")),
        ))
        .add_submenu(Submenu::new(
            "File",
            Menu::new()
                .add_item(CustomMenuItem::new("new-project", "New Project"))
                .add_item(CustomMenuItem::new("open-project", "Open Project")),
        ));

    menu
}

pub fn menu_event_handler(event: WindowMenuEvent) {
    use tauri::{api::dialog::FileDialogBuilder, Manager};

    match event.menu_item_id() {
        "quit" => {
            println!("Quit menu item clicked");
        }
        "new-project" => {
            let file_association = crate::utils::resolve_file_association().unwrap();
            // create a list of extension slices from the file association extension vector
            let extensions: Vec<&str> = file_association
                .extensions
                .iter()
                .map(|extension| extension.as_str())
                .collect();

            FileDialogBuilder::new()
                .add_filter(file_association.name.clone(), &extensions)
                .save_file(move |path| {
                    if path.is_none() {
                        return;
                    }

                    match tauri::async_runtime::block_on(crate::commands::window::open_project(
                        event.window().app_handle(),
                        event.window().state(),
                        path.unwrap().to_str().unwrap().into(),
                    )) {
                        Ok(_) => {}
                        Err(error) => tauri::api::dialog::message(
                            Some(event.window()),
                            "Something went wrong",
                            error,
                        ),
                    }
                })
        }
        "open-project" => {
            let file_association = crate::utils::resolve_file_association().unwrap();
            // create a list of extension slices from the file association extension vector
            let extensions: Vec<&str> = file_association
                .extensions
                .iter()
                .map(|extension| extension.as_str())
                .collect();

            FileDialogBuilder::new()
                .add_filter(file_association.name.clone(), &extensions)
                .pick_file(move |path| {
                    if path.is_none() {
                        return;
                    }

                    match tauri::async_runtime::block_on(crate::commands::window::open_project(
                        event.window().app_handle(),
                        event.window().state(),
                        path.unwrap().to_str().unwrap().into(),
                    )) {
                        Ok(_) => {}
                        Err(error) => tauri::api::dialog::message(
                            Some(event.window()),
                            "Something went wrong",
                            error,
                        ),
                    }
                })
        }
        _ => {
            println!("Unknown menu item clicked");
        }
    }
}
