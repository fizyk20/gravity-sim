use gdk;

pub enum MouseState {
    None,
    Dragging1(f64, f64),
    Dragging2(f64, f64),
}

pub enum MouseResult {
    None,
    Drag1(f64, f64),
    Drag2(f64, f64),
}

impl MouseState {
    pub fn handle_motion(&mut self, state: gdk::ModifierType, x: f64, y: f64) -> MouseResult {
        match *self {
            MouseState::None => {
                if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                    *self = MouseState::Dragging1(x, y);
                } else if state.contains(gdk::ModifierType::BUTTON3_MASK) {
                    *self = MouseState::Dragging2(x, y);
                }
                MouseResult::None
            }
            MouseState::Dragging1(old_x, old_y) => {
                if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                    *self = MouseState::Dragging1(x, y);
                    MouseResult::Drag1(x - old_x, y - old_y)
                } else {
                    *self = MouseState::None;
                    MouseResult::Drag1(x - old_x, y - old_y)
                }
            }
            MouseState::Dragging2(old_x, old_y) => {
                if state.contains(gdk::ModifierType::BUTTON3_MASK) {
                    *self = MouseState::Dragging2(x, y);
                    MouseResult::Drag2(x - old_x, y - old_y)
                } else {
                    *self = MouseState::None;
                    MouseResult::Drag2(x - old_x, y - old_y)
                }
            }
        }
    }
}
