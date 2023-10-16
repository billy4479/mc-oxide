#include "Color.hpp"

Color::Color(QString color) {
    setAutoFillBackground(true);

    auto palette = this->palette();
    palette.setColor(QPalette::ColorRole::Window, QColor(color));
    setPalette(palette);
}