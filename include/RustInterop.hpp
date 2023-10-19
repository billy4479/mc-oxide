#pragma once
#include <lib.rs.h>

inline std::string_view to_string_view(::rust::Str rust_str) {
  return std::string_view(rust_str.data(), rust_str.length());
}

template <typename F> struct NoDiscard {
  F f;
  NoDiscard(F const &func) : f(func) {}
  template <typename... T>
  [[nodiscard]] constexpr auto operator()(T &&...t) const
      noexcept(noexcept(f(std::forward<T>(t)...))) {
    return f(std::forward<T>(t)...);
  }
};

template <typename Ret, typename... Args>
[[nodiscard]] inline constexpr auto to_variant(Ret (*function)(Args...)) {
  if constexpr (std::is_same_v<Ret, void>) {
    return NoDiscard(
        [function](Args... args) -> std::optional<std::exception const *> {
          try {
            if constexpr (sizeof...(args))
              function(std::forward(args...));
            else
              function();
            return {};
          } catch (std::exception const *e) {
            return e;
          }
        });
  } else {
    return NoDiscard(
        [function](Args... args) -> std::variant<Ret, std::exception const *> {
          try {
            if constexpr (sizeof...(args))
              return function(std::forward(args...));
            else
              return function();
          } catch (std::exception const *e) {
            return e;
          }
        });
  }
}

template <typename T> [[nodiscard]] inline constexpr auto must(T result) {
  if constexpr (std::is_same_v<T, std::optional<std::exception const *>>) {
    if (result.has_value()) {
      fmt::println(stderr, "rust exception: {}", result.value()->what());
      std::terminate();
    }

    return;
  } else {
    if (std::holds_alternative<const std::exception *>(result)) {
      fmt::println(stderr, "rust exception: {}",
                   std::get<const std::exception *>(result)->what());
      std::terminate();
    }

    return std::move(std::get<0>(result));
  }
}
