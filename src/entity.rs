use crate::dungeon::{Dungeon, Tile};

use rand::{rngs::StdRng, RngExt};

// structs/enums
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
pub struct Player {
    position: Position,
    health: i32,
}
#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Moving,
    Attack,
}
pub struct Enemy {
    position: Position,
    state: State,
}

impl Player {
    fn new(x: i32, y: i32, health: i32) -> Self {
        let position = Position { x, y };
        Player { position, health}
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }
    // for damage
    pub fn add_health(&mut self, value: i32) {
        self.health += value;
    }
}

pub fn spawn_player(dungeon: &Dungeon, health: i32, rng: &mut StdRng) -> Player {
    loop {
        let x = rng.random_range(0..dungeon.width as i32);
        let y = rng.random_range(0..dungeon.height as i32);

        match dungeon.get(x as usize, y as usize) {
            Tile::Floor => return Player::new(x as i32, y as i32, health),
            Tile::Wall => continue,
        }
    }
}

impl Enemy {
    fn new(x: i32, y: i32) -> Self {
        let position = Position { x, y };
        let state = State::Moving;
        Enemy { position, state }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

pub fn spawn_enemy(dungeon: &Dungeon, rng: &mut StdRng) -> Enemy {
    loop {
        let x = rng.random_range(0..dungeon.width as i32);
        let y = rng.random_range(0..dungeon.height as i32);

        match dungeon.get(x as usize, y as usize) {
            Tile::Floor => return Enemy::new(x as i32, y as i32),
            Tile::Wall => continue,
        }
    }
}