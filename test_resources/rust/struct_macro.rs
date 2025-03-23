#[derive(Debug, Clone, PartialEq, Copy, Hash)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
}

impl Default for Size2f {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serializable)]
pub struct Position2f {
    pub x: f32,
    pub y: f32,
}

impl Default for Position2f {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}
