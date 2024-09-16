#[memory(emplace)]
struct BasicTypesModel {
    byte: u8,
    some_integer: i32,
    some_long: i64,
    some_unsiged_integer: u32,
    some_unsiged_long: u32,
    some_float_number: f32,
    some_double_number: f64,
    some_bool: bool,
    some_string: String,
    vector: Vec<String>,
    generic: LinearTable<f32, Test>,
    custom: MyModel,
    optional_string: Option<String>,
    optional_f32: Option<f32>,
}
