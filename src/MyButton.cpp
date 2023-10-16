#include "MyButton.hpp"

MyButton::MyButton() : QPushButton("Click me!") {
    setSizePolicy(QSizePolicy::Policy::Expanding,
                  QSizePolicy::Policy::Expanding);

    setMaximumHeight(100);
    connect(this, &MyButton::clicked, this, &MyButton::on_clicked);
}

void MyButton::on_clicked() {
    QMessageBox::question(nullptr, "This is a question!",
                          "Do you really want to do this *in C++*???",
                          QMessageBox::StandardButton::Yes |
                              QMessageBox::StandardButton::No);
}
