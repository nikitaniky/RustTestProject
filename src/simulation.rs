use pathfinding::prelude::astar;
use crate::{dungeon::{Dungeon, Tile}, entity::{Player, Enemy, Position, State}};

pub fn enemy_turn(dungeon: &Dungeon, enemy: &mut Enemy, player: &mut Player) -> State {
    let enemy_position = enemy.get_position();
    let player_position = player.get_position();

    let path = path_to_player(get_map(&dungeon), &enemy_position, &player_position);

    match &path {
        Some(p) => {
            if p.len() <= 2 {
                if player.get_health() >= 0 {
                    player.add_health(-5);
                }
                State::Attack
            } else {
                enemy.set_position(get_move_position(enemy_position, path));
                State::Moving
            }
        }
        None => State::Idle,
    }
}

fn get_map(dungeon: &Dungeon) -> Vec<Vec<Tile>>{
    let mut map: Vec<Vec<Tile>> = vec![];
    for y in 0..dungeon.height {
        let mut row: Vec<Tile> = vec![];
        for x in 0..dungeon.width {
            row.push(dungeon.get(x, y));
        }
        map.push(row);
    }
    map
}

fn get_move_position(enemy_position: Position, path: Option<Vec<Position>>) -> Position {
    match path {
        Some(p) if p.len() > 1 => p[1],
        _ => enemy_position,
    }
}

fn path_to_player(map: Vec<Vec<Tile>>, start: &Position, goal: &Position) -> Option<Vec<Position>> {
    let result = astar(
        start,
        |pos: &Position| {
            let dirs = [(1,0), (-1,0), (0,1), (0,-1)];

            dirs.iter()
                .map(|(dx, dy)| Position {
                    x: pos.x + dx,
                    y: pos.y + dy,
                })
                .filter(|p| is_walkable(&map, p.x, p.y))
                .map(|p| (p, 1)) // cost = 1
                .collect::<Vec<_>>()
        },
        |pos: &Position| {
            (goal.x - pos.x).abs() + (goal.y - pos.y).abs()
        },
        |pos: &Position| pos.x == goal.x && pos.y == goal.y,
    );

    result.map(|(path, _cost)| path)
}

fn is_walkable(map: &Vec<Vec<Tile>>, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 { return false }
    let (xu, yu) = (x as usize, y as usize);
    if yu >= map.len() || xu >= map[0].len() { return false }

    map[yu][xu] == Tile::Floor
}