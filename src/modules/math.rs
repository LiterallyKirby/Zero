#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}



pub type Vec2i = Vec2<i32>;
pub type Vec2f = Vec2<f32>;
impl Vec2<i32> {
    pub fn new(value_x: i32, value_y: i32) -> Vec2<i32> {
        Self {
            x: value_x,
            y: value_y,
        }
    }
}

impl Vec2<f32> {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
