This folder contains the algorithms that analyze the images (written in Python and C++). The algorithms are written in Python and C++ and are compiled to a shared library that is used by the Rust backend. 

## Python

If your python code uses any external libraries, you need to add them to the `requirements.txt` file. This file is used by the `src-tauri/build/python.rs` script to install the dependencies.

If you want to add your python code to the build process, you need to add the following line to the `src-tauri/build/main.rs` file:

```rust
    python_builder.add_vendor("NAME_OF_YOUR_PYTHON_FOLDER");
```

For example, if you have a folder called `my_python_code` in the `vendor` folder, you need to add the following line:

```rust
    python_builder.add_vendor("my_python_code");
```

## C++

To build your C++ code, you need to use CMake. The `src-tauri/build/cpp.rs` script will automatically build your CMake project. You need to add a `CMakeLists.txt` file to your C++ folder.

If you want to add your C++ code to the build process, you need to add the following line to the `src-tauri/build/main.rs` file:

```rust
    cpp_builder.add_vendor("NAME_OF_YOUR_CPP_FOLDER");
```

For example, if you have a folder called `my_cpp_code` in the `vendor` folder, you need to add the following line:

```rust
    cpp_builder.add_vendor("my_cpp_code");
```