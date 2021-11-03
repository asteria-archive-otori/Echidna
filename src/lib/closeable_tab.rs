use glib::IsA;
use gtk::{prelude::*, Box, Widget, Button};

pub trait ClosableTabImplementedNotebook {
    fn prepend_closable_page<
        T: IsA<Widget>, 
        U: IsA<Widget>
    >(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32;
    fn create_closable_tab<
        U: IsA<Widget>,

    >(tab_label: Option<&U>) -> (Box, Button);
}

impl ClosableTabImplementedNotebook for gtk::Notebook {
    fn create_closable_tab<
    U: IsA<Widget>,

>(tab_label: Option<&U>)-> (Box, Button) {
        let tab = Box::new(gtk::Orientation::Horizontal, 5);
        if tab_label.is_some() {
            tab.append(tab_label.unwrap());
        }

        let button = gtk::Button::new();
        
        button.set_icon_name("window-close-symbolic");
        button.set_has_frame(false);

        tab.append(&button);

        (tab, button)
    }

    fn prepend_closable_page<T: IsA<Widget>, U: IsA<Widget>>(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32 {
        let (tab, button) = &Self::create_closable_tab(tab_label);
        let page = self.append_page(child, Some(tab));

        button.connect_clicked(glib::clone!(@weak self as notebook => 
            move |_| {
            notebook.remove_page(Some(page));
        }));

        page

    }
}
