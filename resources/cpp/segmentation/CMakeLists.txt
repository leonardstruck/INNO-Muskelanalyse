# Set the minimum version of CMake required to build the project
cmake_minimum_required(VERSION 3.0)

# Set the project name and enable C++ language
project(segmentation)
enable_language(CXX)

# Find OpenCV package
find_package(OpenCV REQUIRED)

# Add the executable
add_executable(segmentation main.cpp)

# Link OpenCV libraries to the executable
target_link_libraries(segmentation PRIVATE ${OpenCV_LIBS})

# Copy OpenCV DLLs to the output directory on Windows
if(WIN32)
    file(MAKE_DIRECTORY $<TARGET_FILE_DIR:segmentation>/bin)
    file(GLOB OPENCV_DLLS "${OpenCV_INSTALL_PATH}/x64/mingw/bin/*.dll")
    foreach(OPENCV_DLL ${OPENCV_DLLS})
        add_custom_command(TARGET segmentation POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${OPENCV_DLL}
            $<TARGET_FILE_DIR:segmentation>)
    endforeach()
endif()

# Copy OpenCV dylibs to the output directory on macOS in a bin subdirectory
if(APPLE)
    file(MAKE_DIRECTORY $<TARGET_FILE_DIR:segmentation>/bin)
    file(GLOB OPENCV_DYLIBS "${OpenCV_INSTALL_PATH}/lib/*.dylib")
    foreach(OPENCV_DYLIB ${OPENCV_DYLIBS})
        add_custom_command(TARGET segmentation POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${OPENCV_DYLIB}
            $<TARGET_FILE_DIR:segmentation>)
    endforeach()
endif()

# Copy OpenCV so files to the output directory on Linux
if(UNIX AND NOT APPLE)
    file(MAKE_DIRECTORY $<TARGET_FILE_DIR:segmentation>/bin)
    file(GLOB OPENCV_SOS "${OpenCV_INSTALL_PATH}/lib/*.so")
    foreach(OPENCV_SO ${OPENCV_SOS})
        add_custom_command(TARGET segmentation POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${OPENCV_SO}
            $<TARGET_FILE_DIR:segmentation>)
    endforeach()
endif()

# Install the executable
install(TARGETS segmentation DESTINATION bin)