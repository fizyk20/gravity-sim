mod simulation;

use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use nalgebra::Vector2;
use simulation::{Body, SimState, G};

fn pericenter(a: f64, ecc: f64) -> f64 {
    a * (1.0 - ecc * ecc)
}

fn vel_at_r(m_center: f64, a: f64, r: f64) -> f64 {
    (G * m_center * (2.0 / r - 1.0 / a)).sqrt()
}

fn prepare_solar_system() -> SimState {
    let mut sim = SimState::new();

    let m_sun = 1.989e6;
    let m_earth = 5.975;
    let m_moon = 0.07348;

    let a_earth = 1496.0;
    let ecc_earth = 0.0167;

    let a_moon = 3.844;
    let ecc_moon = 0.055;

    let p_earth = pericenter(a_earth, ecc_earth);
    let p_moon = pericenter(a_moon, ecc_moon);

    let v_earth = vel_at_r(m_sun, a_earth, p_earth);
    let v_moon = vel_at_r(m_earth, a_moon, p_moon);

    let v_sun = (m_earth * v_earth + m_moon * v_moon) / m_sun;

    let sun = Body {
        mass: 1.989e6,
        pos: Vector2::new(0.0, 0.0),
        vel: Vector2::new(0.0, -v_sun),
    };
    let earth = Body {
        mass: 5.975,
        pos: Vector2::new(p_earth, 0.0),
        vel: Vector2::new(0.0, v_earth),
    };
    let moon = Body {
        mass: 0.07348,
        pos: Vector2::new(p_earth + p_moon, 0.0),
        vel: Vector2::new(0.0, v_earth + v_moon),
    };

    sim.add_body(sun);
    sim.add_body(earth);
    sim.add_body(moon);

    sim
}

fn main() {
    let sim = prepare_solar_system();

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
