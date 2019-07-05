use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::new(None, ApplicationFlags::FLAGS_NONE)
        .expect("Couldn't create a GTK application!");

    app.connect_activate(|app| {
        let win = ApplicationWindow::new(app);

        win.set_title("Gravity simulator");
        win.set_default_size(640, 480);

        win.show_all();
    });

    app.run(&[]);
}
