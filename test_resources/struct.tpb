struct Empty;

struct Test {
    #[0] delta_time: f32,
    #[1] view_width: f32,
    #[2] view_height: f32,
    #[3] touch_start_x: f32,
    #[4] touch_start_y: f32,
    #[5] last_touch_x: f32,
    #[6] last_touch_y: f32,
    #[7] touch_x: f32,
    #[8] touch_y: f32,
}

struct Test {
    #[0] touch_x: f32,
    #[1] touch_y: f32,
    #[2] touch_status: TouchStatus,
}
