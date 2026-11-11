use raylib::prelude::*;
use std::{collections::HashMap};
use crate::sys::{sheets::*, regist::*};


pub struct Renderer{
    textures: HashMap<SheetType, Texture2D>,
    tile_size: f32,
    object_overdraw: usize
}

impl Renderer{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, regist: &Registry) -> Self{
        let mut textures : HashMap<SheetType, Texture2D> = HashMap::new();
        for (kind, def) in regist.sheets.iter(){
            let path = def.path.to_str().unwrap();
            let texture = rl.load_texture(&thread, path).unwrap_or_else(|e| panic!("Failed to load sheet {}: {e} ", path));
            textures.insert(*kind, texture);
        }
        Renderer{textures, tile_size: 32.0, object_overdraw: 3}
    }
}