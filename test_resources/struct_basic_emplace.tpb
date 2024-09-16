#[memory(emplace)]
struct ViewData {
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
