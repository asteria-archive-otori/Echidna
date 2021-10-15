#include "editor.h"
#include <ui_echidna.h>
#include <QtCore>
#include <algorithm>
#include <QtWidgets>

EchidnaEditor::EchidnaEditor(QWidget *parent) : QMainWindow(parent),
                                                ui(new Ui::EchidnaEditor)
{

    ui->setupUi(this);
    this->setCentralWidget(ui->centralWidget);

    QObject::connect(this, &EchidnaEditor::newFileOpened, &EchidnaEditor::handleNewFileOpened);
}

EchidnaEditor::~EchidnaEditor()
{
    delete ui;
};

void EchidnaEditor::actionOpenFolder()
{
    QString dir = QFileDialog::getExistingDirectory(this, 
                "Add a Folder to Workspace",
                QDir::homePath(),
                QFileDialog::ShowDirsOnly);


    std::vector<QFileSystemModel>::iterator foundExistingOpenedFolders = std::find_if(this->folders->begin(), this->folders->end(),
                                                                                      [&](QFileSystemModel *iterator)
                                                                                      {
                                                                                          return iterator->rootPath() == dir;
                                                                                      });
    if (this->folders->end() != foundExistingOpenedFolders)
    {
    }
    else
    {
        QFileSystemModel *model = new QFileSystemModel;
        model->setRootPath(dir);

    }


}

void EchidnaEditor::actionOpenFile(){
    QStringList files = QFileDialog::getOpenFileNames(this,
                    "Open Files",
                    QDir::homePath()
                    );

    for(int i = 0; i < files.size(); ++i){
        emit this->newFileOpened(files.at(i));
    }
}
void EchidnaEditor::handleNewFileOpened(QString filename){

    QFile file(filename);
    


}