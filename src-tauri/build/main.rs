mod cpp;
mod python;
mod utils;

fn main() {
    // check if all build dependencies are installed
    utils::resolve_dependencies();

    // clear the bin directory to avoid outdated files
    utils::clear_bin_dir();

    //
    // BUILD CPP PROJECTS
    //

    let mut cpp_builder = cpp::Builder::new();

    // add projects
    // cpp_builder.add_vendor("PROJECT_FOLDER");
    // The project folder must contain a CMakeLists.txt file

    cpp_builder.add_vendor("segmentation");

    cpp_builder.build();

    //
    // BUILD PYTHON PROJECTS
    //

    let mut python_builder = python::Builder::new();

    // add projects
    // python_builder.add_vendor("PROJECT_FOLDER");
    // The project folder must contain a requirements.txt file if it uses any modules
    // The project folder must contain a main.py file as entry point

    python_builder.add_vendor("analysis");

    python_builder.build();

    //
    // BUILD TAURI
    //

    tauri_build::build()
}
