enum MyEnum {
    #[1] Idle,
    #[2] Move {
        #[1] x: f64,
        #[2] y: f64,
    },
    #[3] Update(#[1] f64, #[2] f64, #[4] String),
}

enum MyEnumWithoutPositions {
    Option1,
    Option2,
    Option3,
    Option4,
}
