#include "MainWindow.hpp"
#include "Color.hpp"
#include "MyButton.hpp"

MainWindow::MainWindow() {
    setWindowTitle("Qt6 in C++");
    setMinimumSize(QSize(800, 600));

#if PLATFORM_LINUX
    setAttribute(Qt::WidgetAttribute::WA_X11NetWmWindowTypeUtility);
#endif

    auto layout = new QVBoxLayout();
    layout->addWidget(new MyButton());

    auto other_layout = new QHBoxLayout();
    other_layout->addWidget(new Color("red"));
    other_layout->addWidget(new Color("blue"));

    auto combo = new QComboBox();
    combo->addItems({"lol", "lmao"});
    combo->setCurrentIndex(-1);

    other_layout->addWidget(combo);

    layout->addLayout(other_layout);

    auto w = new QWidget();
    w->setLayout(layout);
    setCentralWidget(w);
}
