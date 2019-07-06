mod body;

pub use body::Body;
use nalgebra::{DVector, Vector2};
use num::Zero;
use numeric_algs::{State, StateDerivative};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

type Position = Vector2<f64>;
type Velocity = Vector2<f64>;

const DIM: usize = 2;

// Units:
// Length: 1 = 10^8 m (100 000 km)
// Time: 1 = 7 days (a week)
// Mass: 1 = 10^24 kg
pub const G: f64 = 24.412652716032003;

#[derive(Clone)]
pub struct SimState {
    bodies: Vec<Body>,
}

impl SimState {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
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
                let diff = body2.pos - body.pos;
                let dist = body.distance_from(body2);
                let part_accel = G * body2.mass / dist / dist;
                accel += part_accel * diff / dist;
            }
            for j in 0..DIM {
                derivative[i * DIM * 2 + j] = body.vel[j];
                derivative[i * DIM * 2 + DIM + j] = accel[j];
            }
        }
        SimDerivative(DVector::from_vec(derivative))
    }

    pub fn bodies(&self) -> impl Iterator<Item = &Body> {
        self.bodies.iter()
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
