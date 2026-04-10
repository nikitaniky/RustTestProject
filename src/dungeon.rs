use rand::{rngs::StdRng, RngExt};

// structs/enums
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}
pub struct Dungeon {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl Dungeon {
    fn new(width: usize, height: usize) -> Self {
        let tiles = vec![Tile::Wall; width * height];
        Dungeon { width, height, tiles }
    }

    pub fn get(&self, x:usize, y:usize) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y * self.width + x] = tile;
    }
}

pub fn create_dungeon(rng: &mut StdRng) -> Dungeon {
    let mut dungeon = Dungeon::new(20, 20);

    seed_based_generation(&mut dungeon, rng);

    dungeon
}

fn seed_based_generation(dungeon: &mut Dungeon, rng: &mut StdRng) {
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            if rng.random_bool(0.8) {
                dungeon.set(x, y, Tile::Floor);
            }
        }
    }
}