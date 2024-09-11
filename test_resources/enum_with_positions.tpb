enum MyEnum {
    #[3] Idle,
    #[1] Move {
        #[2] x: f64,
        #[1] y: f64,
    },
    #[2] Update(#[8] f64, #[2] f32, #[1] String, #[3] i32),
}
