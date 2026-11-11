// Sys module - contains various systems the game utilizes to function. 
use std::path::PathBuf;
pub mod tile; // Defines WHO and/or WHAT
pub mod map; // Defines WHERE
pub mod sheets;
pub mod regist; // Defines WHY and/or HOW

pub fn res_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        // Dev builds: project root, works no matter where you run cargo from
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res")
    } else {
        // Release builds: folder next to the .exe, for when you ship the game
        std::env::current_exe().unwrap().parent().unwrap().join("res")
    }
}