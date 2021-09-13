#include "editor.h"
#include <ui_echidna.h>

EchidnaEditor::EchidnaEditor(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::EchidnaEditor)
{

ui->setupUi(this);
this->setCentralWidget(ui->centralWidget);



}

EchidnaEditor::~EchidnaEditor(){
    delete ui;
}


/**
 * undefined reference to `vtable for EchidnaEditor'
 * 
 * /


/*

undefined reference to `vtable for EchidnaEditor'

*/