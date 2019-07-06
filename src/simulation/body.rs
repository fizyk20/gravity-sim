use super::{Position, Velocity};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Body {
    pub mass: f64,
    pub pos: Position,
    pub vel: Velocity,

    // data for drawing
    pub radius: f64,
    pub color: (f64, f64, f64),
}

impl Body {
    pub fn distance_from(&self, other: &Body) -> f64 {
        let diff = self.pos - other.pos;
        diff.dot(&diff).sqrt()
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Body {{ m = {}; pos = [{:10.3} ; {:10.3}]; vel = [{:10.3} ; {:10.3}] }}",
            self.mass, self.pos[0], self.pos[1], self.vel[0], self.vel[1]
        )
    }
}
