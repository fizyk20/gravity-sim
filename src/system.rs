use crate::simulation::{Body, SimState};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    exponent: f64,
    grav_const: f64,
    time_scale: f64,
    bodies: Vec<ConfigBody>,
}

#[derive(Clone, Serialize, Deserialize)]
struct ConfigBody {
    name: String,
    mass: f64,
    pos: (f64, f64),
    vel: (f64, f64),
    radius: f64,
    color: (f64, f64, f64),
}

impl ConfigBody {
    fn into_body(self) -> Body {
        Body {
            name: self.name,
            mass: self.mass,
            pos: Vector2::new(self.pos.0, self.pos.1),
            vel: Vector2::new(self.vel.0, self.vel.1),
            radius: self.radius,
            color: self.color,
        }
    }
}

#[allow(unused)]
fn pericenter(a: f64, ecc: f64) -> f64 {
    a * (1.0 - ecc)
}

#[allow(unused)]
fn vel_at_r(m_center: f64, a: f64, r: f64) -> f64 {
    (m_center * (2.0 / r - 1.0 / a)).sqrt()
}

pub fn create_solar_system() -> SimState {
    let mut file = File::open("config.yaml")
        .ok()
        .expect("Couldn't open config.yaml!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config = serde_yaml::from_str::<Config>(&contents).unwrap();

    let mut sim = SimState::new(config.grav_const, config.exponent, config.time_scale);

    for body in config.bodies {
        sim.add_body(body.into_body());
    }

    sim.adjust_for_center_of_mass();

    sim
}
