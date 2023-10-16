cmake_minimum_required(VERSION 3.18)

project(
  mc-oxide-qt
  VERSION 0.0.1
  LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)
include(Cache)
include(Sanitizers)
include(Warnings)
include(BuildCargo)
include(Dependencies)

find_package(Qt6 REQUIRED COMPONENTS Core Widgets Gui)

qt_standard_project_setup()

file(GLOB_RECURSE ${PROJECT_NAME}_SRC CONFIGURE_DEPENDS src/*.cpp src/*.hpp
     include/*.hpp)

qt_add_executable(${PROJECT_NAME} ${${PROJECT_NAME}_SRC})

target_include_directories(${PROJECT_NAME} PRIVATE include)

target_link_libraries(${PROJECT_NAME} PRIVATE RustLib Qt6::Core Qt6::Widgets
                                              Qt6::Gui fmt)

if(UNIX)
  find_package(OpenSSL REQUIRED) # For rust libs

  target_link_libraries(${PROJECT_NAME} PRIVATE OpenSSL::SSL)
endif()

target_precompile_headers(${PROJECT_NAME} PRIVATE "include/Precompiled.hpp")

enable_sanitizers(${PROJECT_NAME})
set_project_warnings(${PROJECT_NAME})

set_target_properties(${PROJECT_NAME} PROPERTIES WIN32_EXECUTABLE ON
                                                 MACOSX_BUNDLE ON)
