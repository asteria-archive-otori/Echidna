/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod file;
pub mod imp;
pub mod menubar;
use crate::prelude::*;
use glib::object::{Cast, IsA};
use gtk::subclass::prelude::*;
use std::{error::Error, fmt};

glib::wrapper! {
    pub struct EchidnaWindow(ObjectSubclass<imp::EchidnaWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl EchidnaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let win: Self =
            glib::Object::new(&[("application", &application)]).expect("can't make window");

        win
    }

    pub fn to_imp(&self) -> &imp::EchidnaWindow {
        imp::EchidnaWindow::from_instance(self)
    }

    pub fn get_current_tab<A: IsA<gtk::Widget>>(&self) -> Result<A, Box<dyn Error>> {
        let window_imp = self.to_imp();
        let tab_bar = &window_imp.tab_bar;
        let view = tab_bar.view().expect("No view in tab barr");
        let page = view.selected_page();

        match page {
            None => {
                #[derive(Debug)]
                struct Error {}

                impl fmt::Display for Error {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "No tabs are currently opened, maybe there are no tabs.")
                    }
                }

                impl std::error::Error for Error {}

                Err(Box::new(Error {}))
            }
            Some(page) => match page.child().downcast::<A>() {
                Ok(page) => Ok(page),
                Err(widget) => {
                    #[derive(Debug)]
                    struct Error {
                        widget: gtk::Widget,
                    }

                    impl std::error::Error for Error {}

                    impl fmt::Display for Error {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            write!(f, "Cannot downcast {:#?} to type parameter A. Maybe it's not in the type you are looking for.", self.widget)
                        }
                    }

                    Err(Box::new(Error { widget }))
                }
            },
        }
    }
}
