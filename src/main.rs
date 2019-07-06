mod mouse_state;
mod renderer;
mod simulation;

use gdk;
use gio::prelude::*;
use gio::ApplicationFlags;
use glib;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow};

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use mouse_state::MouseState;
use nalgebra::Vector2;
use numeric_algs::integration::{Integrator, RK4Integrator, StepSize};
use renderer::Renderer;
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
        radius: 7.0,
        color: (1.0, 1.0, 0.0),
    };
    let earth = Body {
        mass: 5.975,
        pos: Vector2::new(p_earth, 0.0),
        vel: Vector2::new(0.0, v_earth),
        radius: 1.0,
        color: (0.0, 1.0, 1.0),
    };
    let moon = Body {
        mass: 0.07348,
        pos: Vector2::new(p_earth + p_moon, 0.0),
        vel: Vector2::new(0.0, v_earth + v_moon),
        radius: 1.0,
        color: (0.5, 0.5, 0.5),
    };

    let test = Body {
        mass: 0.01,
        pos: Vector2::new(500.0, 0.0),
        vel: Vector2::new(0.0, 393.0),
        radius: 2.0,
        color: (0.5, 0.5, 0.5),
    };

    sim.add_body(sun);
    sim.add_body(earth);
    sim.add_body(moon);
    sim.add_body(test);

    sim
}

fn build_ui(app: &Application, mut sim: SimState) {
    let win = ApplicationWindow::new(app);
    let renderer_rc = Rc::new(RefCell::new(Renderer::new(sim.clone(), 0.0, 0.0)));
    let mouse_state = Rc::new(RefCell::new(MouseState::None));

    win.set_title("Gravity simulator");
    win.set_default_size(640, 480);

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let mut integrator = RK4Integrator::new(0.1);
        let mut prev_step = Instant::now();
        loop {
            let now = Instant::now();
            let time_diff = now - prev_step;
            prev_step = now;
            let time_diff = time_diff.as_secs() as f64 + time_diff.subsec_nanos() as f64 / 1e9;
            integrator.propagate_in_place(
                &mut sim,
                SimState::derivative,
                StepSize::Step(time_diff),
            );
            let _ = tx.send(sim.clone());
        }
    });

    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_events(
        gdk::EventMask::POINTER_MOTION_MASK
            | gdk::EventMask::POINTER_MOTION_HINT_MASK
            | gdk::EventMask::BUTTON_PRESS_MASK,
    );
    let drawing_area_clone = drawing_area.clone();

    let renderer1 = renderer_rc.clone();
    rx.attach(None, move |sim_state| {
        renderer1.borrow_mut().update_state(sim_state);
        drawing_area_clone.queue_draw();

        glib::Continue(true)
    });

    let renderer2 = renderer_rc.clone();
    drawing_area.connect_draw(move |area, cr| {
        let w = area.get_allocated_width() as f64;
        let h = area.get_allocated_height() as f64;
        renderer2.borrow_mut().update_dimensions(w, h);

        renderer2.borrow().render(cr);

        glib::signal::Inhibit(true)
    });

    let renderer3 = renderer_rc.clone();
    let mouse_state1 = mouse_state.clone();
    drawing_area.connect_motion_notify_event(move |_area, event| {
        let (pos_x, pos_y) = event.get_position();
        if let Some((dx, dy)) =
            mouse_state1
                .borrow_mut()
                .handle_motion(event.get_state(), pos_x, pos_y)
        {
            renderer3.borrow_mut().shift_center(dx, dy);
        }

        if event.get_is_hint() {
            event.request_motions();
        }

        glib::signal::Inhibit(true)
    });

    win.add(&drawing_area);

    win.show_all();
}

fn main() {
    let sim = prepare_solar_system();

    let app = Application::new(None, ApplicationFlags::FLAGS_NONE)
        .expect("Couldn't create a GTK application!");

    app.connect_activate(move |app| build_ui(app, sim.clone()));

    app.run(&[]);
}
