/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::file::FileImplementedEditor;
use super::EchidnaWindow;
use crate::components::GettingStartedPage;
use crate::prelude::*;
use gio::{MenuModel, SimpleAction};
use glib::clone;
use gtk::prelude::*;
use gtk::AboutDialog;
pub trait MenubarImplementedEditor {
    fn setup_menubar(&self);
}

impl MenubarImplementedEditor for EchidnaWindow {
    fn setup_menubar(&self) {
        let app = self
            .application()
            .expect("self does not have an application set.");
        let menubuilder = gtk::Builder::from_string(include_str!("./menu.ui"));
        let menubar: MenuModel = menubuilder
            .object("menu")
            .expect("Could not get object 'menu' from builder.");
        app.set_menubar(Some(&menubar));
        self.set_show_menubar(true);
        {
            let act_exit: SimpleAction = SimpleAction::new("exit", None);
            app.add_action(&act_exit);

            act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                    app.quit();
                }
            ));
        }
        {
            let act_about: SimpleAction = SimpleAction::new("about", None);
            app.add_action(&act_about);
            act_about.connect_activate(|_action, _value| {
                let about_dialog: AboutDialog = AboutDialog::new();

                about_dialog.set_license_type(gtk::License::Mpl20);
                about_dialog.set_program_name(Some("Echidna Code Editor"));
                about_dialog.set_website(Some("https://gitlab.com/EchidnaHQ/Echidna"));
                about_dialog.set_authors(&["FortressValkriye"]);
                about_dialog.set_copyright(Some("Made with by ❤️ Echidna contributors"));
                about_dialog.set_visible(true);
            });
        }
        {
            //app.notebook = Some(Rc::new(RefCell::new(notebook)));
            let act_exit: SimpleAction = SimpleAction::new("exit", None);
            app.add_action(&act_exit);

            act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                    app.quit();
                }
            ));
        }
        {
            let act_about: SimpleAction = SimpleAction::new("about", None);
            app.add_action(&act_about);
            act_about.connect_activate(|_action, _value| {
                let about_dialog: AboutDialog = AboutDialog::new();

                about_dialog.set_license_type(gtk::License::Mpl20);
                about_dialog.set_program_name(Some("Echidna Code Editor"));
                about_dialog.set_website(Some("https://github.com/EchidnaHQ/Echidna"));
                about_dialog.set_authors(&["FortressValkriye"]);
                about_dialog.set_copyright(Some("Made with by ❤️ Echidna contributors"));
                about_dialog.set_visible(true);
            });
        }
        {
            let act_report_issue = SimpleAction::new("report-issue", None);

            app.add_action(&act_report_issue);

            act_report_issue.connect_activate(|_action, _variant| {
                webbrowser::open("https://github.com/EchidnaHQ/Echidna/issues/new");
            });
        }
        {
            let act_search_feature_requests = SimpleAction::new("search-feature-requests", None);

            app.add_action(&act_search_feature_requests);

            act_search_feature_requests.connect_activate(|_action, _variant| {
            webbrowser::open("https://github.com/EchidnaHQ/Echidna/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement");
        });
        }
        {
            let act_window_close = SimpleAction::new("close", None);

            self.add_action(&act_window_close);
            let window = self.clone();

            act_window_close.connect_activate(move |_action, _variant| {
                window.close();
            });
        }
        {
            let action_open_file: SimpleAction = SimpleAction::new("open-file", None);

            self.add_action(&action_open_file);
            action_open_file.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                    window.action_open_file();
            }));
        }
        {
            let action_save_file_as = SimpleAction::new("save-file-as", None);

            self.add_action(&action_save_file_as);

            action_save_file_as.connect_activate(clone!(@weak self as window =>
            move |_action, _variant| {
                window.action_save_file_as();
            }));
        }
        {
            let action_getting_started = SimpleAction::new("get-started", None);

            self.add_action(&action_getting_started);

            action_getting_started.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                    let page = GettingStartedPage::new();
                    page.setup_actions(&window);
                    window.to_imp().notebook.prepend_closable_page(&page, Some(&gtk::Label::new(Some(&"Getting Started"))));

                }));
        }
    }
}
