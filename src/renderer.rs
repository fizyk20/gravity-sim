use crate::simulation::SimState;
use cairo::Context;
use std::f64::consts::PI;

pub struct Renderer {
    state: SimState,
    da_width: f64,
    da_height: f64,
}

impl Renderer {
    pub fn new(state: SimState, da_width: f64, da_height: f64) -> Self {
        Self {
            state,
            da_width,
            da_height,
        }
    }

    pub fn update_state(&mut self, new_state: SimState) {
        self.state = new_state;
    }

    pub fn update_dimensions(&mut self, width: f64, height: f64) {
        self.da_width = width;
        self.da_height = height;
    }

    fn scale(&self) -> f64 {
        let smaller_dim = if self.da_width < self.da_height {
            self.da_width
        } else {
            self.da_height
        };
        smaller_dim / 4000.0
    }

    // converts sim coordinates to drawing area coordinates
    fn sim_to_da(&self, x: f64, y: f64) -> (f64, f64) {
        let center_x = 0.0;
        let center_y = 0.0;

        let da_x = (x - center_x) * self.scale() + self.da_width / 2.0;
        let da_y = (center_y - y) * self.scale() + self.da_height / 2.0;

        (da_x, da_y)
    }

    pub fn render(&self, cr: &Context) {
        let w = self.da_width;
        let h = self.da_height;

        cr.rectangle(0.0, 0.0, w, h);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.fill();

        for body in self.state.bodies() {
            let (x, y) = self.sim_to_da(body.pos[0], body.pos[1]);
            let radius = body.radius * self.scale() * 3.0;

            cr.set_source_rgb(body.color.0, body.color.1, body.color.2);
            cr.arc(x, y, radius, 0.0, 2.0 * PI);
            cr.fill();

            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.5);
            cr.arc(x, y, radius, 0.0, 2.0 * PI);
            cr.stroke();
        }
    }
}
