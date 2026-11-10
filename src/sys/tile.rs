

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Terrain{
    Grass,
    Dirt,
    Water
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Object{
    Empty,
    Tree,
    Forest,
    Clump,
    Wall
}

#[derive(Clone, Copy)]
pub struct Tile{
    pub terrain: Terrain,
    pub object: Object
}

impl Default for Tile{
    fn default() -> Self{
        Tile {terrain: Terrain::Grass, object: Object::Empty}
    }
}