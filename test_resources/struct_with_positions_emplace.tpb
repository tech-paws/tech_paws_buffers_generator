#[memory(emplace)]
struct Test {
    #[2] touch_x: f32,
    #[1] touch_y: f32,
    #[3] touch_status: TouchStatus,
}
