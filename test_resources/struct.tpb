struct Empty;

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

struct Test {
    #[2] touch_x: f32,
    #[1] touch_y: f32,
    #[3] touch_status: TouchStatus,
}

struct GenericType {
    items: Vec<Test>,
    table: LinearTable<f32, Test>,
}
