/// Top level doc comment
/// Some description

#[namespace = "test"]

/// Some doc comment
/// Another doc comment
enum MyEnum {
    /// This is Idle
    #[1] Idle,
    /// This is Move!
    #[2] Move {
        /// This is x field
        #[1] x: f64,
        /// This is y field
        #[2] y: f64,
    },
    /// This is Update case
    #[3] Update(
        /// This is first option
        #[1] f64,
        /// This is second option
        #[2] f64,
        /// This is third option
        #[4] String
    ),
}

/// Hello World!
/// This is View Data, Important Structure!
struct ViewData {
    /// Delta time is delta time
    #[1] delta_time: f32,
    /// View Width
    #[2] view_width: f32,
    /// View Height!
    #[3] view_height: f32,
    /// Touch Start X
    /// It is starting position
    #[4] touch_start_x: f32,
    /// Touch Start Y
    /// It is starting position
    #[5] touch_start_y: f32,
}

/// Say hello returns hello [name]! string.
///
/// # Panic
///
/// Don't worry, this function doesn't panic!!
fn say_hello(name: String) -> String;

/// Get up to date view data frame.
signal view_data -> ViewData;
