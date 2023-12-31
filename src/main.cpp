#include "MainWindow.hpp"
#include "RustInterop.hpp"

int main(int argc, char **argv) {
  must(to_variant(try_init_logger)());

  auto config = must(to_variant(try_load_config)());

  auto dir = to_string_view(config_get_server_dir(*config));

  log_info(fmt::format("Hello from C++! server_dir: {}", dir));

  auto app = QApplication(argc, argv);

  auto window = MainWindow();
  window.show();

  return app.exec();
}