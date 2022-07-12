#[derive(Debug, Clone, PartialEq)]
pub enum MyEnum {
    Idle,
    Move {
        x: f64,
        y: f64,
    },
    Update(
        f64,
        f64,
        String,
    ),
}

impl Default for MyEnum {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MyEnumWithoutPositions {
    Option1,
    Option2,
    Option3,
    Option4,
}

impl Default for MyEnumWithoutPositions {
    fn default() -> Self {
        Self::Option1
    }
}
