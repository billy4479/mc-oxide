#pragma once
#include <lib.rs.h>

inline std::string_view to_string_view(::rust::Str rust_str) {
  return std::string_view(rust_str.data(), rust_str.length());
}