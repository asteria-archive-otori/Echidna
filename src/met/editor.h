#include <QMainWindow>
#include <QtWidgets>
#include <vector>
namespace Ui
{
    class EchidnaEditor;
}

class EchidnaEditor : public QMainWindow
{
    Q_OBJECT

public:
    explicit EchidnaEditor(QWidget *parent = nullptr);
    ~EchidnaEditor();

private slots:

    void actionOpenFolder();
    void actionAddFolderToWorkspace(QString folder);
    void actionOpenFile();
    void handleNewFileOpened(QString file);

signals:

    void newFileOpened(QString file);
    void newFolderOpened();
private:
    Ui::EchidnaEditor *ui;
    std::vector<QFileSystemModel> *folders;
    std::vector<QFile> *files;
};







