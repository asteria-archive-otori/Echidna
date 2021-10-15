
/*

    Echidna Core Text Editor. This is the Monaco replacement for Echidna.

    A lot of the code are ~~stolen~~ inspired from [Qt's Code Editor example](https://doc.qt.io/qt-5/qtwidgets-widgets-codeeditor-example.html), with added features to replicate the Visual Studio Code experience.

*/

#include <QtWidgets>

class LineNumberArea : public QWidget
{

public:
    LineNumberArea(EchidnaCoreEditor *editor) : QWidget(editor), coreEditor(editor)
    {
    }

    QSize sizeHint() const override
    {
        return QSize(coreEditor->lineNumberAreaWidth(), 0);
    }

protected:
    void paintEvent(QPaintEvent *event) override
    {
        coreEditor->lineNumberAreaPaintEvent(event);
    }

    EchidnaCoreEditor *coreEditor;
};

class EchidnaCoreEditor : public QPlainTextEdit
{
    Q_OBJECT

public:
    EchidnaCoreEditor(QWidget *parent = nullptr);
    ~EchidnaCoreEditor();

    void lineNumberAreaPaintEvent(QPaintEvent *event);
    int lineNumberAreaWidth();
protected:
    void resizeEvent(QResizeEvent *event) override;

private slots:
    void highlightCurrentLine();
    void updateLineNumberAreaWidth(int newBlockCount);
    void updateLineNumberArea(const QRect &rect, int dy);

    
    LineNumberArea *lineNumber;
};




