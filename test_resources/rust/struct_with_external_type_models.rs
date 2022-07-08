#[derive(Debug, Clone, PartialEq)]
pub struct Test {
    touch_x: f32,
    touch_y: f32,
    touch_status: TouchStatus,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            touch_x: 0.0,
            touch_y: 0.0,
            touch_status: TouchStatus::default(),
        }
    }
}
