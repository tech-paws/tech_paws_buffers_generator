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
