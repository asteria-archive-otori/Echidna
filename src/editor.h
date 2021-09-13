#include <QMainWindow>

namespace Ui {
class EchidnaEditor;
}


class EchidnaEditor : public QMainWindow
{
    Q_OBJECT

public:
    explicit EchidnaEditor(QWidget *parent = nullptr);
    ~EchidnaEditor();

private slots:
   void addFolderIntoWorkspace(){

   }

private:
    Ui::EchidnaEditor *ui;
    
};