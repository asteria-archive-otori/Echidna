#include "core-editor.h"

EchidnaCoreEditor::EchidnaCoreEditor(QWidget *parent) : QPlainTextEdit(parent) {
    LineNumberArea *lineNumber = new LineNumberArea(this);

    QObject::connect(this, &EchidnaCoreEditor::cursorPositionChanged, &EchidnaCoreEditor::highlightCurrentLine);
    QObject::connect(this, &EchidnaCoreEditor::updateRequest, &EchidnaCoreEditor::updateLineNumberArea);
    QObject::connect(this, &EchidnaCoreEditor::blockCountChanged, &EchidnaCoreEditor::updateLineNumberAreaWidth);

    this->highlightCurrentLine();
    this->updateLineNumberAreaWidth(0);
}

EchidnaCoreEditor::~EchidnaCoreEditor(){
    
}

int EchidnaCoreEditor::lineNumberAreaWidth(){

}


void EchidnaCoreEditor::lineNumberAreaPaintEvent(QPaintEvent *event){

}

void EchidnaCoreEditor::resizeEvent(QResizeEvent *e){

    QPlainTextEdit::resizeEvent(e);

    QRect contentsRect = this->contentsRect();
    

}

    /**
     *
     * This function implements the functionaly that highlight the current line your cursor is currently on.
     * 
     * TODO: Implement support for multi-line editing.
     *
     */
void EchidnaCoreEditor::highlightCurrentLine(){
   
    if(this->isReadOnly()){
        return;
    } else {
        QList<QTextEdit::ExtraSelection> selections;

        QTextEdit::ExtraSelection selection;

        selection.format.setProperty(QTextFormat::FullWidthSelection, true);

        QBrush selectionFormatBrush = selection.format.foreground();

        
    }
    
}



/*

the method `set_menubar` exists for reference `&components::echidna_editor::imp::EchidnaEditor`, but its trait bounds were not satisfied

method cannot be called on `&components::echidna_editor::imp::EchidnaEditor` due to unsatisfied trait bounds

note: the following trait bounds were not satisfied:
      `components::echidna_editor::imp::EchidnaEditor: glib::IsA<gtk4::Application>`
      which is required by `components::echidna_editor::imp::EchidnaEditor: gtk4::prelude::GtkApplicationExt`
      `&components::echidna_editor::imp::EchidnaEditor: glib::IsA<gtk4::Application>`
      which is required by `&components::echidna_editor::imp::EchidnaEditor: gtk4::prelude::GtkApplicationExt`

*/
