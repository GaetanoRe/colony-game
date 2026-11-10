pub mod rects;
pub mod renderer;
use std::{path::PathBuf};

// Render constants
pub const SIZE:f32 = 32.0;

pub fn res_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        // Dev builds: project root, works no matter where you run cargo from
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res")
    } else {
        // Release builds: folder next to the .exe, for when you ship the game
        std::env::current_exe().unwrap().parent().unwrap().join("res")
    }
}