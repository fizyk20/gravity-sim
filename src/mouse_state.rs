use gdk;

pub enum MouseState {
    None,
    Drag1(f64, f64),
}

impl MouseState {
    pub fn handle_motion(
        &mut self,
        state: gdk::ModifierType,
        x: f64,
        y: f64,
    ) -> Option<(f64, f64)> {
        match *self {
            MouseState::None => {
                if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                    *self = MouseState::Drag1(x, y);
                }
                None
            }
            MouseState::Drag1(old_x, old_y) => {
                if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                    *self = MouseState::Drag1(x, y);
                    Some((x - old_x, y - old_y))
                } else {
                    *self = MouseState::None;
                    Some((x - old_x, y - old_y))
                }
            }
        }
    }
}
