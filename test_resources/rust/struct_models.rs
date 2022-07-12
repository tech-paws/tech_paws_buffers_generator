#[derive(Debug, Clone, PartialEq)]
pub struct Empty;

impl Default for Empty {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewData {
    pub delta_time: f32,
    pub view_width: f32,
    pub view_height: f32,
    pub touch_start_x: f32,
    pub touch_start_y: f32,
    pub last_touch_x: f32,
    pub last_touch_y: f32,
    pub touch_x: f32,
    pub touch_y: f32,
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            view_width: 0.0,
            view_height: 0.0,
            touch_start_x: 0.0,
            touch_start_y: 0.0,
            last_touch_x: 0.0,
            last_touch_y: 0.0,
            touch_x: 0.0,
            touch_y: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Test {
    pub touch_x: f32,
    pub touch_y: f32,
    pub touch_status: TouchStatus,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            touch_x: 0.0,
            touch_y: 0.0,
            touch_status: TouchStatus::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
    pub items: Vec<Test>,
    pub table: LinearTable<f32, Test>,
}

impl Default for GenericType {
    fn default() -> Self {
        Self {
            items: Vec::<Test>::default(),
            table: LinearTable::<f32, Test>::default(),
        }
    }
}
