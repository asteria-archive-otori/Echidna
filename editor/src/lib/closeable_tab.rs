/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::components::tab_label::TabLabel;
use crate::prelude::*;
use glib::IsA;
use gtk::Widget;

pub trait ClosableTabImplementedNotebook {
    fn prepend_closable_page<T: IsA<Widget>, U: IsA<Widget>>(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32;
    fn append_closable_page<T: IsA<Widget>, U: IsA<Widget>>(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32;
}

impl ClosableTabImplementedNotebook for gtk::Notebook {
    fn prepend_closable_page<T: IsA<Widget>, U: IsA<Widget>>(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32 {
        let tab_label_widget = TabLabel::new(tab_label);
        let page = self.prepend_page(child, Some(&tab_label_widget));

        tab_label_widget
            .to_imp()
            .button
            .connect_clicked(glib::clone!(@weak self as notebook =>
                move |_| {
                notebook.remove_page(Some(page));
            }));

        page
    }

    fn append_closable_page<T: IsA<Widget>, U: IsA<Widget>>(
        &self,
        child: &T,
        tab_label: Option<&U>,
    ) -> u32 {
        let tab_label_widget = TabLabel::new(tab_label);
        let page = self.append_page(child, Some(&tab_label_widget));

        tab_label_widget
            .to_imp()
            .button
            .connect_clicked(glib::clone!(@weak self as notebook =>
                move |_| {
                notebook.remove_page(Some(page));
            }));

        page
    }
}
