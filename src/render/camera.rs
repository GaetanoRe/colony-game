use crate::sys::{input::FrameInput};
pub struct GameCam {
    pub target: (f32, f32),
    pub zoom: f32,
    pub speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32
}

impl GameCam{
    pub fn new(target: (f32, f32)) -> Self{
        GameCam{target, zoom: 1.0, speed: 400.0, min_zoom: 0.25, max_zoom: 4.0}
    }

    pub fn update(&mut self, input: &FrameInput, delta:f32, bounds:(f32, f32)){
        // Update movement and camera logic here
    }

    pub fn screen_to_world(&self, screen: (f32, f32), screen_size: (f32, f32)) -> (f32, f32){
        (
            (screen.0 - screen_size.0 / 2.0) / self.zoom + self.target.0,
            (screen.1 - screen_size.1 / 2.0) / self.zoom + self.target.1
        )
    }
}