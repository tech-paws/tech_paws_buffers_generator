#[derive(Debug, Clone, PartialEq)]
pub enum ComplexEnum {
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
