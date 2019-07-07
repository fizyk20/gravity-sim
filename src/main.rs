mod interface;
mod mouse_state;
mod renderer;
mod simulation;
mod system;

use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::Application;

use interface::build_ui;
use system::create_solar_system;

fn main() {
    let sim = create_solar_system();

    let app = Application::new(None, ApplicationFlags::FLAGS_NONE)
        .expect("Couldn't create a GTK application!");

    app.connect_activate(move |app| build_ui(app, sim.clone()));

    app.run(&[]);
}
