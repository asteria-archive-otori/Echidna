use gtk::{prelude::*, FileChooserAction, FileChooserNative, NativeDialog, ResponseType};
use std::cell::RefCell;

thread_local! {
    static DIALOGS: RefCell<Vec<NativeDialog>> = RefCell::new(Vec::new());
}

///
/// A lovely abstraction over [gtk::FileChooserNative]. 
/// 
/// Since FileChooserNative is not a top-level widget, GTK doesn't keep it alive, so due how Rust works, the window will automatically close when being opened. (it'll feel like it doesn't open at all!).
/// 
/// This function takes care of keeping it alive by pushing the dialog to a Vec stored with `thread_local`. It'll automatically remove the dialog from the Vec when the dialog is closed.
/// 
pub fn file_chooser<F>(
    title: Option<&str>,
    parent: Option<&impl gtk::glib::IsA<gtk::Window>>,
    action: FileChooserAction,
    accept_label: Option<&str>,
    cancel_label: Option<&str>,
    handler: F,
) where
    F: 'static + Fn(&FileChooserNative, ResponseType) -> (),
{
    /*
       Borrows DIALOGS vector mutably, create a new dialog, and push that dialog into the vector.
       This is required because we own the dialog and thus the dialog will be destroyed when this function has completed.
    */
    DIALOGS.with(|dialogs| {
        let dialogs = &mut *dialogs.borrow_mut();

        let dialog = FileChooserNative::new(title, parent, action, accept_label, cancel_label);

        let dialog_clone = dialog.clone();
        // The upcast() function moves the dialog variable, so we need to get a reference to the dialog trough dialog_clone
        dialogs.push(dialog.upcast::<gtk::NativeDialog>());

        dialog_clone.connect_response(move |dialog, response| {
            handler(dialog, response);
            dialog.destroy();
            DIALOGS.with(|dialogs| {
                let dialogs = &mut *dialogs.borrow_mut();

                if let Some(position) = dialogs
                    .iter()
                    .position(|item| *item == dialog.clone().upcast::<gtk::NativeDialog>())
                {
                    dialogs.remove(position);
                }
            });
        });
        dialog_clone.show();
    });
}
