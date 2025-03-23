#[derive(Copy, Hash)]
struct Size2f {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy, Serializable)]
struct Position2f {
    x: f32,
    y: f32,
}
