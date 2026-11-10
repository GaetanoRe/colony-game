use super::{tile::*, sheets::*};
use std::{collections::HashMap, path::PathBuf};

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
    pub terrain: HashMap<Terrain, TerrainDef>,
    pub objects: HashMap<Object, ObjectDef>,
    pub sheets: SheetTable
}

impl Registry{
    pub fn new() -> Self{
        // Place every default sheet in the sheet table
        let sheet_table = SheetTable::with_defaults();

        // Setup the registry
        let terrain = HashMap::from([
            (Terrain::Grass, TerrainDef{
                name: "Grass", 
                walkable: true, 
                sprite: SheetCoords { 
                    col: 0, 
                    row: 4, 
                    width: 1, 
                    height: 1,
                    kind: SheetType::Terrain
                }
            }),
            (Terrain::Dirt, TerrainDef{
                name: "Dirt",
                walkable: true,
                sprite: SheetCoords { 
                    col: 2, 
                    row: 2, 
                    width: 1, 
                    height: 1,
                    kind: SheetType::Terrain
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
                    kind: SheetType::Terrain
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
                    kind: SheetType::Trees
                })
            })
        ]);
       
        
        Registry{terrain, objects, sheets: sheet_table}
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