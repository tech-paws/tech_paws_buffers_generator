#[derive(Debug, Clone, PartialEq)]
pub struct Test {
    delta_time: f32,
    view_width: f32,
    view_height: f32,
    touch_start_x: f32,
    touch_start_y: f32,
    last_touch_x: f32,
    last_touch_y: f32,
    touch_x: f32,
    touch_y: f32,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            view_width: 0.0,
            view_height: 0.0,
            touch_start_x: 0.0,
            touch_start_y: 0.0,
            last_touch_x: 0.0,
            last_touch_y: 0.0,
            touch_x: 0.0,
            touch_y: 0.0,
        }
    }
}
