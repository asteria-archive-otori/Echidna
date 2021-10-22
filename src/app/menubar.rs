use super::imp::EchidnaEditor;
use super::imp::EchidnaEditorExt;
use gio::{MenuModel, SimpleAction};
use glib::clone;
use gtk::prelude::*;
use gtk::AboutDialog;

pub trait MenubarImplementedEditor {
    fn setup_menubar(
        &self,
        app: &super::EchidnaEditor,
        window: &gtk::ApplicationWindow,
        builder: &gtk::Builder,
    );
}

impl MenubarImplementedEditor for EchidnaEditor {
    fn setup_menubar(
        &self,
        app: &super::EchidnaEditor,
        window: &gtk::ApplicationWindow,
        builder: &gtk::Builder,
    ) {
        let menubuilder = gtk::Builder::from_string(include_str!("../../ui/menu.ui"));
        let menubar: MenuModel = menubuilder
            .object("menu")
            .expect("Could not get object 'menu' from builder.");
        app.set_menubar(Some(&menubar));
        window.set_show_menubar(true);
        let act_exit: SimpleAction = SimpleAction::new("exit", None);
        app.add_action(&act_exit);

        act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                        app.quit();
                }
        ));

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

        let notebook: gtk::Notebook = builder
            .object("echidna-notebook")
            .expect("Could not get 'echidna-notebook' from builder.");
        //app.notebook = Some(Rc::new(RefCell::new(notebook)));
        let act_exit: SimpleAction = SimpleAction::new("exit", None);
        app.add_action(&act_exit);

        act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                        app.quit();
                }
        ));

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

        let act_report_issue = SimpleAction::new("report-issue", None);

        app.add_action(&act_report_issue);

        act_report_issue.connect_activate(|_action, _variant| {
            webbrowser::open("https://github.com/EchidnaHQ/Echidna/issues/new");
        });
        let act_search_feature_requests = SimpleAction::new("search-feature-requests", None);

        app.add_action(&act_search_feature_requests);

        act_search_feature_requests.connect_activate(|_action, _variant| {
            webbrowser::open("https://github.com/EchidnaHQ/Echidna/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement");
        });

        let act_window_close = SimpleAction::new("close", None);

        window.add_action(&act_window_close);

        act_window_close.connect_activate(clone!(@weak window =>
            move | _action, _variant | {
                window.close();
            }
        ));

        let action_open_file: SimpleAction = SimpleAction::new("open-file", None);

        window.add_action(&action_open_file);
        action_open_file.connect_activate(clone!(@weak window, @weak app, @weak notebook =>
                move |action, variant| {
                Self::action_open_file(window, app, action, variant, notebook);
        }));
    }
}
