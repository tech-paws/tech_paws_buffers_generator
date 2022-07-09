#[namespace = "test"]

// My Fancy Enum
enum MyEnum {
    #[1] Idle,
    #[2] Move {
        #[1] x: f64,
        #[2] y: f64,
    },
    #[3] Update(#[1] f64, #[2] f64, #[4] String),
}

/*
 Hello World!
 */
struct ViewData {
    #[1]  delta_time: f32,
    #[2]  view_width: f32,
    #[3]  view_height: f32,
    #[4]  touch_start_x: f32,
    #[5]  touch_start_y: f32,
    #[6]  last_touch_x: f32,
    #[7]  last_touch_y: f32,
    #[8]  touch_x: f32,
    #[9]  touch_y: f32,
    #[10] touch_status: TouchStatus,
}

fn say_hello(name: String) -> String;
