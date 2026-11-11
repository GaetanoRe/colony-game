use raylib::prelude::Rectangle; 
use super::SIZE;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RectType{
    Terrain,
    Object
}

pub struct Rect{
    col:i32,
    row:i32,
    width:i32,
    height:i32,
    kind: RectType
}

impl Rect{
    pub fn new(col:i32, row:i32, width:i32, height:i32, kind: RectType) -> Self{
        Rect { col, row, width, height, kind }
    }

    pub fn to_rectangle(&self) -> Rectangle{
        let s = if self.kind == RectType::Terrain {SIZE} else {SIZE/ 2.0};
        Rectangle::new(self.col as f32 * s, self.row as f32 * s, self.width as f32 * s, self.height as f32 * s)
    }
}
