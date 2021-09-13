#include <QApplication>
#include "editor.h"

int main(int argc, char *argv[]){

    QApplication EchidnaEditorApp(argc, argv);

    EchidnaEditor Editor;

    Editor.show();

    EchidnaEditorApp.exec();

}