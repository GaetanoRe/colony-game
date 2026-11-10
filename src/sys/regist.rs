use super::{tile::*, res_dir};
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sheet{
    Terrain,
    Trees
}

pub struct SheetCoords{
    col:i32,
    row:i32,
    width:i32,
    height:i32,
    sheet: Sheet
}

pub struct ObjectDef{
    pub name : & 'static str,
    pub walkable: bool,
    pub sprite: Option<SheetCoords>
    
}

pub struct TerrainDef{
    pub name: &'static str,
    pub walkable: bool,
    pub sprite: SheetCoords
}

pub struct Registry{
    terrain: HashMap<Terrain, TerrainDef>,
    objects: HashMap<Object, ObjectDef>
}

impl Registry{
    pub fn new() -> Self{
        let terrain_path: PathBuf = res_dir().join("terrain");
        let object_path :PathBuf = res_dir().join("objects");
        let terrain = HashMap::from([
            (Terrain::Grass, TerrainDef{
                name: "Grass", 
                walkable: true, 
                sprite: SheetCoords { 
                    col: 0, 
                    row: 4, 
                    width: 1, 
                    height: 1,
                    sheet: Sheet::Terrain
                }
            }),
            (Terrain::Dirt, TerrainDef{
                name: "Dirt",
                walkable: false,
                sprite: SheetCoords { 
                    col: 2, 
                    row: 2, 
                    width: 1, 
                    height: 1,
                    sheet: Sheet::Terrain
                }
            }),
            (Terrain::Water, TerrainDef{
                name:"Water",
                walkable: false,
                sprite: SheetCoords { 
                    col: 0, 
                    row: 0, 
                    width: 1, 
                    height: 1,
                    sheet: Sheet::Terrain
                }

            })
        ]);

        let objects = HashMap::from([
            (Object::Empty, ObjectDef{name: "Empty", walkable: true, sprite: None }),
            (Object::Tree, ObjectDef{
                name: "Pine Tree",
                walkable: false,
                sprite: Some(SheetCoords{ 
                    col: 3, 
                    row: 12, 
                    width:2, 
                    height: 3,
                    sheet: Sheet::Trees
                })
            })
        ]);

        Registry{terrain, objects}
    }

    pub fn object(&self, o: Object) -> &ObjectDef{
        &self.objects[&o]
    }

    pub fn terrain(&self, t: Terrain) -> &TerrainDef{
        &self.terrain[&t]
    }

    pub fn is_walkable(&self, tile: Tile) -> bool{
        self.terrain(tile.terrain).walkable && self.object(tile.object).walkable
    }

    

}