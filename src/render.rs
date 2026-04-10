use crate::{dungeon::{Dungeon, Tile}, entity::{Enemy, Player, Position}};

pub fn render(dungeon: &Dungeon, player: &Player, enemies: &Vec<Enemy>) {
    print_player_health(&player);
    print_dungeon(dungeon, player, enemies);
}

fn print_player_health(player: &Player) {
    println!("Health: {}/100", player.get_health())
}

fn print_dungeon(dungeon: &Dungeon, player: &Player, enemies: &Vec<Enemy>) {
    let p_position= player.get_position();
    let e_positions = get_enemy_position_vector(enemies);

    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            print!("{}", get_tile(dungeon, &p_position, &e_positions, x, y));
        }
        println!();
    }

    println!("\n");
}

fn get_enemy_position_vector(enemies: &Vec<Enemy>) -> Vec<Position> {
    let mut positions: Vec<Position> = vec![];
    
    for enemy in enemies {
        positions.push(enemy.get_position());
    }
    positions
}

fn get_tile(dungeon: &Dungeon, p_position: &Position, e_positions: &Vec<Position>, x: usize, y: usize) -> char {
    let tile;

    if p_position.x == x as i32 && p_position.y == y as i32 {
        tile = 'P';
    } else if is_enemy_position_equal(&e_positions, x, y) {
        tile = 'E'
    } else {
        tile = match dungeon.get(x, y) {
            Tile::Wall => '#',
            Tile::Floor => '.',
        };
    }
    tile
}

fn is_enemy_position_equal(e_positions: &Vec<Position>, x: usize, y: usize) -> bool {
    for e_position in e_positions {
        if e_position.x == x as i32 && e_position.y == y as i32 {
            return true
        }
    }
    false
}