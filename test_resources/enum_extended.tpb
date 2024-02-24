enum MyEnum {
    #[1] Idle,
    #[2] Move {
        #[1] x: f64,
        #[2] y: f64,
    },
    #[3] Update(#[1] f64, #[2] f64, #[4] String),
}

enum MyEnumWithoutPositions {
    Option1(u64),
    Option2 { name: String },
    Option3,
    Option4,
}

enum MyEnumWithManyArguments {
    Option1(u64, u64, u64),
    Option2 { name: String },
    Option3,
    Option4,
}

enum MyEnumWithNamedArguments {
    Option1 {
        name: String,
        value: f32,
        bytes: Vec<u8>
    },
    Option2(u64, u64, u64),
    Option3,
    Option4,
}
