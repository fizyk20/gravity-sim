use crate::simulation::SimState;
use cairo::Context;
use nalgebra::Vector2;
use std::collections::VecDeque;
use std::f64::consts::PI;

pub enum SceneCenter {
    CenterOfMass(f64, f64),
    Body(usize, f64, f64),
}

const MAX_PATH_LEN: usize = 1_000;

pub struct Renderer {
    state: SimState,
    path_history: Vec<VecDeque<Vector2<f64>>>,
    da_width: f64,
    da_height: f64,
    center: SceneCenter,
    length_scale: f64,
}

impl Renderer {
    pub fn new(state: SimState, da_width: f64, da_height: f64) -> Self {
        let mut path_history = Vec::new();
        for body in state.bodies() {
            let mut path = VecDeque::new();
            path.push_back(body.pos);
            path_history.push(path);
        }
        Self {
            state,
            path_history,
            da_width,
            da_height,
            center: SceneCenter::CenterOfMass(0.0, 0.0),
            length_scale: 4e8,
        }
    }

    pub fn update_state(&mut self, new_state: SimState) {
        self.state = new_state;
        for (i, body) in self.state.bodies().enumerate() {
            let last_pos = self.path_history[i].back();
            if last_pos
                .map(|last_pos| {
                    let diff = body.pos - last_pos;
                    let len = diff.dot(&diff).sqrt();
                    len > 1e6
                })
                .unwrap_or(true)
            {
                self.path_history[i].push_back(body.pos);
                if self.path_history[i].len() > MAX_PATH_LEN {
                    let _ = self.path_history[i].pop_front();
                }
            }
        }
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
        smaller_dim / self.length_scale
    }

    fn center(&self) -> (f64, f64) {
        match self.center {
            SceneCenter::CenterOfMass(x, y) => (x, y),
            SceneCenter::Body(i, add_x, add_y) => {
                let body = self.state.get_body(i);
                (body.pos[0] + add_x, body.pos[1] + add_y)
            }
        }
    }

    pub fn shift_center(&mut self, dx: f64, dy: f64) {
        let dx = -dx / self.scale();
        let dy = dy / self.scale();

        match self.center {
            SceneCenter::CenterOfMass(ref mut x, ref mut y)
            | SceneCenter::Body(_, ref mut x, ref mut y) => {
                *x += dx;
                *y += dy;
            }
        }
    }

    pub fn change_zoom(&mut self, dy: f64) {
        self.length_scale *= (dy / 400.0).exp();
    }

    fn get_body_by_name(&self, name: &str) -> Option<usize> {
        self.state
            .bodies()
            .enumerate()
            .find(|(_, body)| &body.name == name)
            .map(|(i, _)| i)
    }

    pub fn set_reference(&mut self, reference: &str) {
        if let Some(body) = self.get_body_by_name(reference) {
            self.center = SceneCenter::Body(body, 0.0, 0.0);
        } else {
            self.center = SceneCenter::CenterOfMass(0.0, 0.0);
        }
    }

    // converts sim coordinates to drawing area coordinates
    fn sim_to_da(&self, x: f64, y: f64) -> (f64, f64) {
        let (center_x, center_y) = self.center();

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

        for (i, body) in self.state.bodies().enumerate() {
            cr.set_source_rgb(body.color.0, body.color.1, body.color.2);
            cr.set_line_width(0.5);

            // draw path
            let init_pos = self.path_history[i][0];
            let (x, y) = self.sim_to_da(init_pos[0], init_pos[1]);
            cr.move_to(x, y);

            for pos in &self.path_history[i] {
                let (x, y) = self.sim_to_da(pos[0], pos[1]);
                cr.line_to(x, y);
            }
            let (x, y) = self.sim_to_da(body.pos[0], body.pos[1]);
            cr.line_to(x, y);

            cr.stroke();

            // draw the body
            let radius = body.radius * self.scale() * 3.0;

            cr.arc(x, y, radius, 0.0, 2.0 * PI);
            cr.fill();

            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.5);
            cr.arc(x, y, radius, 0.0, 2.0 * PI);
            cr.stroke();
        }
    }
}
