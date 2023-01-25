cmake_minimum_required(VERSION 3.0.0)
project(segmentation VERSION 0.1.0)

include(CTest)
enable_testing()


set(OpenCV_STATIC OFF)
find_package( OpenCV REQUIRED )
include_directories( ${OpenCV_INCLUDE_DIRS} )


add_executable(segmentation main.cpp)

target_link_libraries( segmentation ${OpenCV_LIBS} )

if(NOT OpenCV_STATIC)
    if(WIN32)
        if(MSVC AND OpenCV_RUNTIME AND OpenCV_ARCH)
            set(OpenCV_LIB_PATH "${OpenCV_CONFIG_PATH}/${OpenCV_ARCH}/${OpenCV_RUNTIME}/bin")
            file(GLOB OPENCV_DLL_FILES ${OpenCV_LIB_PATH}/*.*dll)
            file(COPY ${OPENCV_DLL_FILES} DESTINATION ${CMAKE_BINARY_DIR})
        else()
            message(STATUS "Failed to find OpenCV binary folder. Please copy the dll files manually!")
        endif()
    endif()
endif()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

install(TARGETS segmentation DESTINATION bin)