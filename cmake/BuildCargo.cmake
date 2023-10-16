set(CARGO_BUILD_DIR ${PROJECT_SOURCE_DIR}/target)

set(RUST_LIB_NAME
    ${CARGO_BUILD_DIR}/$<IF:$<CONFIG:Debug>,debug,release>/libcxx_bindings${CMAKE_STATIC_LIBRARY_SUFFIX}
)

set(RUST_CXX_BINDINGS_DIR ${CARGO_BUILD_DIR}/cxxbridge/cxx-bindings/src)

file(GLOB_RECURSE RUST_SRC CONFIGURE_DEPENDS
     ${PROJECT_SOURCE_DIR}/crates/**/*.rs)

add_custom_command(
  OUTPUT ${RUST_LIB_NAME}
  COMMAND cargo build $<IF:$<CONFIG:Debug>,,--release>
  DEPENDS ${RUST_SRC})

add_custom_target(CargoBuildOutput DEPENDS ${RUST_LIB_NAME})

add_library(RustLib STATIC ${RUST_CXX_BINDINGS_DIR}/lib.rs.cc)

add_dependencies(RustLib CargoBuildOutput)

target_include_directories(RustLib PUBLIC ${RUST_CXX_BINDINGS_DIR})
target_link_libraries(RustLib PUBLIC ${RUST_LIB_NAME})
