use std::collections::HashMap;

use raylib::{error::GenMeshError::Allocation, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action{
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight
}

pub struct FrameInput{
    pub camera_dir: (f32, f32),
    pub zoom: f32,
    pub click: Option<(f32,f32)>
}


pub struct Input{
    bindings: HashMap<Action, Vec<KeyboardKey>>
}

impl Input{
    pub fn new() -> Self {
        let bindings = HashMap::from([
            (Action::CameraUp, vec![KeyboardKey::KEY_W, KeyboardKey::KEY_UP]),
            (Action::CameraDown, vec![KeyboardKey::KEY_S, KeyboardKey::KEY_DOWN]),
            (Action::CameraLeft, vec![KeyboardKey::KEY_A, KeyboardKey::KEY_LEFT]),
            (Action::CameraRight, vec![KeyboardKey::KEY_D, KeyboardKey::KEY_RIGHT])
        ]);

        Input{bindings: bindings}
    }
    
    // If the button was held
    pub fn is_down(&self, rl: &RaylibHandle, action: Action) -> bool{
        self.bindings
            .get(&action)
            .map_or(false, |keys| keys.iter().any(|&k| rl.is_key_down(k)))
    }

    // If the button was pressed
    pub fn is_pressed(&self, rl: &RaylibHandle, action: Action) -> bool{
        self.bindings
            .get(&action)
            .map_or(false, |keys| keys.iter().any(|&k| rl.is_key_pressed(k)))
    }



    pub fn rebind(&mut self, action: Action, keys:Vec<KeyboardKey>){
        self.bindings.insert(action, keys);
    }

    pub fn input_on_axis(&self, rl: &RaylibHandle, negative: Action, positive:Action) -> f32{
        let mut value = 0.0;
        if(self.is_down(rl, negative)){value -= 1.0;}
        else if(self.is_down(rl, positive)){value += 1.0;}
        value
    }

    pub fn poll(&self, rl:&RaylibHandle) -> FrameInput{
        let camera_dir = (self.input_on_axis(rl, Action::CameraLeft, Action::CameraRight), self.input_on_axis(rl, Action::CameraUp, Action::CameraDown));
        let wheel = rl.get_mouse_wheel_move();
        let zoom = if wheel > 0.0 {1.1} else if wheel < 0.0 {1.0 / 1.1} else {1.0};

        let click = if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){ let p = rl.get_mouse_position(); Some((p.x, p.y))} else{None};
        
        FrameInput {camera_dir, zoom, click}
    }
}

