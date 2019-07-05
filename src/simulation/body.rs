use super::{Position, Velocity};

#[derive(Clone, Copy)]
pub struct Body {
    pub mass: f64,
    pub pos: Position,
    pub vel: Velocity,
}

impl Body {
    pub fn distance_from(&self, other: &Body) -> f64 {
        let diff = self.pos - other.pos;
        diff.dot(&diff).sqrt()
    }
}
