/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use super::super::window::menubar::MenubarImplementedEditor;
use super::super::window::EchidnaWindow;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::Application;

#[derive(Debug, Default)]
pub struct EchidnaEditor {
    pub name: &'static str,
    pub app_id: &'static str,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaEditor {
    const NAME: &'static str = "EchidnaEditorApplication";
    type Type = super::EchidnaEditor;
    type ParentType = Application;

    fn new() -> Self {
        Self {
            name: "Echidna Code Editor",
            app_id: "land.echidna.editor",
        }
    }
}

impl ObjectImpl for EchidnaEditor {}

impl ApplicationImpl for EchidnaEditor {
    fn activate(&self, app: &Self::Type) {
        let window = EchidnaWindow::new(app);

        window.setup_menubar();
        window.set_application(Some(app));

        window.show();
    }
}

impl GtkApplicationImpl for EchidnaEditor {}
