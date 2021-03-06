mod body;

pub use body::Body;
use glib;
use nalgebra::{DVector, Vector2};
use num::Zero;
use numeric_algs::{
    integration::{Integrator, RK4Integrator, StepSize},
    State, StateDerivative,
};

use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::thread;
use std::time::{Duration, Instant};

type Position = Vector2<f64>;
type Velocity = Vector2<f64>;

const DIM: usize = 2;

#[derive(Clone)]
pub struct SimState {
    exponent: f64,
    grav_const: f64,
    time_scale: f64,
    bodies: Vec<Body>,
    turn_off: HashSet<(usize, usize)>,
}

impl SimState {
    pub fn new(grav_const: f64, exponent: f64, time_scale: f64) -> Self {
        Self {
            exponent,
            grav_const,
            time_scale,
            bodies: Vec::new(),
            turn_off: HashSet::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn body_by_name(&self, name: &str) -> Option<usize> {
        self.bodies
            .iter()
            .enumerate()
            .find(|(_, body)| body.name == name)
            .map(|(idx, _)| idx)
    }

    pub fn turn_off(&mut self, name1: &str, name2: &str) {
        let _ = self.body_by_name(name1).and_then(|body1| {
            self.body_by_name(name2).map(|body2| {
                let _ = self.turn_off.insert((body1, body2));
            })
        });
    }

    pub fn derivative(&self) -> SimDerivative {
        let mut derivative = Vec::with_capacity(self.bodies.len() * DIM * 2);
        for _ in 0..2 * DIM * self.bodies.len() {
            derivative.push(0.0);
        }
        for (i, body) in self.bodies.iter().enumerate() {
            let mut accel: Vector2<f64> = Zero::zero();
            for (i2, body2) in self.bodies.iter().enumerate() {
                if i2 == i {
                    continue;
                }
                // Turn off interactions between selected pairs
                if self.turn_off.contains(&(i, i2)) || self.turn_off.contains(&(i2, i)) {
                    continue;
                }
                let diff = body2.pos - body.pos;
                let dist = body.distance_from(body2);
                // correction by 1e3 needed because dist is in km, and grav_const assumes meters
                let part_accel = self.grav_const * body2.mass * (dist * 1e3).powf(self.exponent);
                accel += part_accel * diff / dist;
            }
            for j in 0..DIM {
                derivative[i * DIM * 2 + j] = body.vel[j] * self.time_scale;
                // like above - all distances and velocities are in km (km/s), but acceleration
                // comes out in m/s^2
                derivative[i * DIM * 2 + DIM + j] = accel[j] * self.time_scale / 1e3;
            }
        }
        SimDerivative(DVector::from_vec(derivative))
    }

    pub fn adjust_for_center_of_mass(&mut self) {
        let mut total_mom = Vector2::new(0.0, 0.0);
        let mut total_mass = 0.0;
        for body in &self.bodies {
            total_mom += body.vel * body.mass;
            total_mass += body.mass;
        }

        for body in &mut self.bodies {
            body.vel -= total_mom / total_mass;
        }
    }

    pub fn bodies(&self) -> impl Iterator<Item = &Body> {
        self.bodies.iter()
    }

    pub fn get_body(&self, idx: usize) -> &Body {
        &self.bodies[idx]
    }
}

impl State for SimState {
    type Derivative = SimDerivative;

    fn shift_in_place(&mut self, dir: &SimDerivative, amount: f64) {
        for (i, body) in self.bodies.iter_mut().enumerate() {
            for j in 0..DIM {
                body.pos[j] += dir.0[i * DIM * 2 + j] * amount;
                body.vel[j] += dir.0[i * DIM * 2 + DIM + j] * amount;
            }
        }
    }
}

impl fmt::Debug for SimState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for (i, body) in self.bodies.iter().enumerate() {
            writeln!(formatter, "{}. {:?}", i + 1, body)?;
        }
        Ok(())
    }
}

const FRAME: f64 = 0.016;

fn seconds(duration: Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

pub fn start_simulation(tx: glib::Sender<SimState>, mut sim: SimState) {
    thread::spawn(move || {
        let mut integrator = RK4Integrator::new(0.1);
        let mut prev_step = Instant::now();
        let mut prev_frame = Instant::now();
        loop {
            let now = Instant::now();
            let time_diff = now - prev_step;
            prev_step = now;
            let time_diff = seconds(time_diff);
            integrator.propagate_in_place(
                &mut sim,
                SimState::derivative,
                StepSize::Step(time_diff),
            );
            if seconds(now - prev_frame) > FRAME {
                let _ = tx.send(sim.clone());
                prev_frame = now;
            }
        }
    });
}

#[derive(Clone)]
pub struct SimDerivative(DVector<f64>);

impl Add<SimDerivative> for SimDerivative {
    type Output = SimDerivative;

    fn add(self, other: SimDerivative) -> SimDerivative {
        SimDerivative(self.0 + other.0)
    }
}

impl Sub<SimDerivative> for SimDerivative {
    type Output = SimDerivative;

    fn sub(self, other: SimDerivative) -> SimDerivative {
        SimDerivative(self.0 - other.0)
    }
}

impl Mul<f64> for SimDerivative {
    type Output = SimDerivative;

    fn mul(self, other: f64) -> SimDerivative {
        SimDerivative(self.0 * other)
    }
}

impl Div<f64> for SimDerivative {
    type Output = SimDerivative;

    fn div(self, other: f64) -> SimDerivative {
        SimDerivative(self.0 / other)
    }
}

impl Neg for SimDerivative {
    type Output = SimDerivative;

    fn neg(self) -> SimDerivative {
        SimDerivative(-self.0)
    }
}

impl StateDerivative for SimDerivative {
    fn abs(&self) -> f64 {
        self.0.dot(&self.0).sqrt()
    }
}
