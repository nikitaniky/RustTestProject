mod dungeon;
mod entity;
mod render;
mod simulation;

use crate::{dungeon::create_dungeon, entity::{Enemy, State, spawn_enemy, spawn_player}};

use std::{io, thread::sleep, time::Duration};
use rand::{rngs::StdRng, SeedableRng};
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

fn main() {
    // get seed
    println!("Dungeon Seed:");
    let mut rng = StdRng::seed_from_u64(string_to_u64(&read_string()));
    
    // creation of world and entities
    let dungeon = create_dungeon(&mut rng);
    let mut player = spawn_player(&dungeon, 100, &mut rng);
    let mut enemies = vec![];
    for _ in 0..3 {
        enemies.push(spawn_enemy(&dungeon, &mut rng));
    }

    // game loop
    loop {
        render::render(&dungeon, &player, &enemies); // print health and dungeon

        // loop ends
        if all_enemies_idle(&enemies) {
            println!("Enemy couldn't reach you...");
            break;
        } else if player.get_health() <= 0 {
            println!("You died!");
            break;
        }

        // enemy logic
        for mut enemy in &mut enemies {
            let state = simulation::enemy_turn(&dungeon, &mut enemy, &mut player);
            enemy.set_state(state);
        }
        
        sleep(Duration::from_millis(500))
    }
}

fn read_string() -> String {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => return input,
            Err(_) => continue,
        }
    }
}

fn string_to_u64(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn all_enemies_idle(enemies: &Vec<Enemy>) -> bool {
    for enemy in enemies {
        match enemy.get_state() {
            State::Moving => return false,
            State::Attack => return false,
            _ => {}
        }
    }
    true
}