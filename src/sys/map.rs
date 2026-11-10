use super::tile::*;
use noise::{NoiseFn, OpenSimplex};


pub struct Map{
    pub width: usize,
    pub height: usize,
    grid: Vec<Tile>
}

impl Map{
    pub fn new(width:usize, height:usize) -> Self{
        Map{
            width,
            height,
            grid: vec![Tile::default(); width * height]
        }
    }

    fn index(&self, x: usize, y: usize) -> usize{
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Tile{
        self.grid[self.index(x,y)]
    }

    pub fn get_mut(&mut self, x:usize, y:usize) -> &mut Tile{
        let i = self.index(x,y);
        &mut self.grid[i]
    }

    pub fn generate(&mut self, seed: u32, scale:f64){
        let noise = OpenSimplex::new(seed);

        // First Pass, Simple Gen
        for y in 0..self.height{
            for x in 0..self.width{
                let value = noise.get([x as f64 / scale, y as f64 / scale]);
                let terr = if value < -0.2{
                    Terrain::Water
                } else if value < -1.0{
                    Terrain::Dirt
                } else{
                    Terrain::Grass
                };

                self.get_mut(x,y).terrain = terr;
            }
        }

        // Second Pass, Fine details

    }

    pub fn populate(&mut self, seed: u32, kind:Object, scale:f64){
        let noise = OpenSimplex::new(seed);

        for y in 0..self.height{
            for x in 0..self.width{
                let value = noise.get([x as f64 / scale, y as f64 / scale]);
                let can_place = value > 0.3 && self.get(x,y).terrain == Terrain::Grass;
                self.get_mut(x,y).object = if can_place {kind} else {Object::Empty}
            }
        }
    }
}

