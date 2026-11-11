// The modules utilized in this project
mod sys;
mod render;
mod engine;

// Imports
use rand::random_range;
use raylib::{prelude::*};
use sys::{regist::*,map::*,tile::*};
use render::{*,renderer::*, camera::*};
use engine::input::*;

// Constants, things that should not change

// Window Constants
const WIN_HEIGHT:i32 = 720;
const WIN_WIDTH:i32 = 1280;
const MAP_WIDTH:usize = 300;
const MAP_HEIGHT:usize = 300;
const FPS:u32 = 60;






fn main() {
    // Systems initialize!!!
    let registry = Registry::new();
    let world_seed:u32 = random_range(..5000);
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    map.generate(world_seed, 10.0);
    map.populate(world_seed/2, Object::Tree, 9.0);

    // Setup RayblibHandle and the Raylib Thread. This will also create a window.
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("ColonyGame")
        .build();

    rl.set_target_fps(FPS);

    // Plug raylib stuff into renderer
    let renderer = Renderer::new(&mut rl, &thread, &registry);

    // Input and camera initialization
    let input_map: Input = Input::new();
    let bounds: (f32, f32) = (MAP_WIDTH as f32 * SIZE, MAP_HEIGHT as f32 * SIZE);
    let mut game_camera:GameCam = GameCam::new((bounds.0 / 2.0, bounds.1 / 2.0));
    


    while !rl.window_should_close() {
        // Update loop values
        let delta = rl.get_frame_time(); // Get Frame Time
        let frame_input = input_map.poll(&rl); // Poll for imputs
        game_camera.update(&frame_input, delta, bounds); // Update the camera logically

        // Begin drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello from raylib-rs!!!", 12, 12, 20, Color::BLACK);
    }
}









