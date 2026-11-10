mod sys;
mod render;

use raylib::{ffi::Rectangle, prelude::*};
use std::{collections::HashMap, path::PathBuf};
use rand::Rng;
use sys::map::Map;
use render::rects::RectType;
use render::rects::Rect;



// Constants, things that should not change

// Window Constants
const WIN_HEIGHT:i32 = 720;
const WIN_WIDTH:i32 = 1280;
const FPS:u32 = 60;






fn main() {
    // Paths needed for textures
    

    // World information
    let map = Map::new(100, 100);



    // Setup RayblibHandle and the Raylib Thread. This will also create a window.
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("Hello Raylib")
        .build();

    rl.set_target_fps(FPS);


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello from raylib-rs!!!", 12, 12, 20, Color::BLACK);
    }
}









