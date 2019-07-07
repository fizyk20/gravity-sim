use crate::simulation::{Body, SimState, G};
use nalgebra::Vector2;

fn pericenter(a: f64, ecc: f64) -> f64 {
    a * (1.0 - ecc * ecc)
}

fn vel_at_r(m_center: f64, a: f64, r: f64) -> f64 {
    (G * m_center * (2.0 / r - 1.0 / a)).sqrt()
}

pub fn create_solar_system() -> SimState {
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
        name: "Sun".to_owned(),
        mass: 1.989e6,
        pos: Vector2::new(0.0, 0.0),
        vel: Vector2::new(0.0, -v_sun),
        radius: 7.0,
        color: (1.0, 1.0, 0.0),
    };
    let earth = Body {
        name: "Earth".to_owned(),
        mass: 5.975,
        pos: Vector2::new(p_earth, 0.0),
        vel: Vector2::new(0.0, v_earth),
        radius: 0.064,
        color: (0.0, 1.0, 1.0),
    };
    let moon = Body {
        name: "Moon".to_owned(),
        mass: 0.07348,
        pos: Vector2::new(p_earth + p_moon, 0.0),
        vel: Vector2::new(0.0, v_earth + v_moon),
        radius: 0.017,
        color: (0.5, 0.5, 0.5),
    };

    let test = Body {
        name: "Test body".to_owned(),
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

    sim.adjust_for_center_of_mass();

    sim
}
