use std::{collections::HashMap, path::PathBuf};
use crate::sys::res_dir;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SheetType{
    Terrain,
    Trees
}

pub struct SheetCoords{
    pub col:i32,
    pub row:i32,
    pub width:i32,
    pub height:i32,
    pub kind: SheetType
}

pub struct SheetDef{
    pub path : PathBuf,
    pub cell_size: u32
}

pub struct SheetTable{
    sheets: HashMap<SheetType, SheetDef>
}

impl SheetTable {
    pub fn new() -> Self {
        SheetTable{sheets: HashMap::new()}
    }

    pub fn with_defaults() -> Self {
        let mut table = SheetTable::new();
        table.add_sheet(SheetType::Terrain, "terrain/terrain_tiles24.png", 32);
        table.add_sheet(SheetType::Trees, "objects/trees/summer_pine_tree_tiles.png", 16);
        table
    }

    pub fn add_sheet(&mut self, kind: SheetType, path:&str, size: u32){
        let d = res_dir().join(path);
        self.sheets.insert(kind, SheetDef{path: d, cell_size: size});
    }

    // Lets the caller handle a missing sheet
    pub fn get(&self, kind: SheetType) -> Option<&SheetDef> {
        self.sheets.get(&kind)
    }

    pub fn iter(&self) ->impl Iterator<Item = (&SheetType, &SheetDef)>{
        self.sheets.iter()
    }
}



