use crate::engine::{input::FrameInput};
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

        // The bounds are in pixels, so remember to multiply the bounds by size before putting it in here.

        // Clamp the zoom between x0.1 and x4.0
        self.zoom = f32::clamp(self.zoom * input.zoom,self.min_zoom, self.max_zoom);

        // Create the x and y force utilizing the direction of input, the speed of the camera, and the delta time
        // and divide them both by the zoom so that the speed can scale with the zoom.
        let force_x = input.camera_dir.0 * self.speed * delta / self.zoom;
        let force_y = input.camera_dir.1 * self.speed * delta / self.zoom;

        // Apply the target force and clamp it between the bounds given to this function
        self.target.0 = f32::clamp(self.target.0 + force_x, 0.0, bounds.0);
        self.target.1 = f32::clamp(self.target.1 + force_y, 0.0, bounds.1);

    }

    pub fn screen_to_world(&self, screen: (f32, f32), screen_size: (f32, f32)) -> (f32, f32){
        (
            (screen.0 - screen_size.0 / 2.0) / self.zoom + self.target.0,
            (screen.1 - screen_size.1 / 2.0) / self.zoom + self.target.1
        )
    }
}