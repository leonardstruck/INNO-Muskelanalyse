mod cpp;
mod utils;

fn main() {
    // clear the bin directory to avoid outdated files
    utils::clear_bin_dir();

    //
    // BUILD CPP PROJECTS
    //

    let mut cpp_builder = cpp::Builder::new();

    // add projects
    // cpp_builder.add_vendor("PROJECT_FOLDER");
    // The project folder must contain a CMakeLists.txt file

    cpp_builder.add_vendor("preprocessing");

    cpp_builder.build();

    //
    // BUILD TAURI
    //

    tauri_build::build()
}
