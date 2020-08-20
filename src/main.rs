use std::collections::HashMap;
use std::io;

// struct Portal {
//     exit: Location,
// }

// enum Corner {
//     Northeast,
//     Southeast,
//     Southwest,
//     Northwest,
// }

enum Terrain {
    Hole,
    Ground,
    // Trap,
    // Quicksand,
    // Water,
    // Spring,
    // Portal,
}

struct Tile {
    terrain: Terrain,
    // elevation: i32,
    // corner: Option<Corner>,
}

#[derive(Clone)]
pub struct Move {
    distance: i32,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Location {
    x: i32,
    y: i32,
}

fn interpret_map_and_moves(
    map_lines: Vec<&str>,
    move_lines: Vec<&str>,
) -> (HashMap<Location, Tile>, Vec<Move>) {
    let mut map: HashMap<Location, Tile> = HashMap::new();
    for line in map_lines {
        let items: Vec<&str> = line.split(",").collect();
        if items[0] == "hole" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Hole,
                    // elevation: items[3].parse::<i32>().unwrap(),
                    // corner: None,
                },
            );
        } else if items[0] == "ground" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Ground,
                    // elevation: items[3].parse::<i32>().unwrap(),
                    // corner: None,
                },
            );
        }
    }
    let moves: Vec<Move> = move_lines
        .iter()
        .map(|m| {
            let s: Vec<&str> = m.split(",").collect();
            return Move {
                distance: s[0].parse::<i32>().unwrap(),
            };
        })
        .collect();
    return (map, moves);
}

fn try_moves_to_reach_hole(
    map: &HashMap<Location, Tile>,
    position: &Location,
    moves: &Vec<Move>,
) -> Option<Vec<(i32, Direction)>> {
    // check whether hole reached
    if let Some(current_tile) = map.get(position) {
        let in_hole = match current_tile.terrain {
            Terrain::Hole => true,
            _ => false,
        };
        if in_hole {
            return Some(vec![]);
        }
    }

    // try all moves in all directions
    for i in 0..moves.len() {
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        {
            if let Some(end_position) = try_move(&map, position, &moves[i], &direction) {
                let mut remaining_moves = moves.clone();
                remaining_moves.remove(i);
                if let Some(mut moves_to_solve) =
                    try_moves_to_reach_hole(&map, &end_position, &remaining_moves)
                {
                    moves_to_solve.insert(0, (i as i32, direction.clone()));
                    return Some(moves_to_solve);
                }
            }
        }
    }
    return None;
}

// attempts to move with the nominated put/direction
// returns the (optional) finishing position, None indicates failure
fn try_move(
    map: &HashMap<Location, Tile>,
    position: &Location,
    chosen_move: &Move,
    chosen_direction: &Direction,
) -> Option<Location> {
    let mut move_copy = chosen_move.clone();
    let mut position_copy = position.clone();
    let cur_tile = map.get(&position_copy);
    while move_copy.distance > 0 && cur_tile.is_some() {
        match chosen_direction {
            Direction::Up => position_copy.y += 1,
            Direction::Down => position_copy.y -= 1,
            Direction::Left => position_copy.x -= 1,
            Direction::Right => position_copy.x += 1,
        }
        move_copy.distance -= 1;
    }
    // println!("{} {}", position_copy.x, position_copy.y);
    return Some(position_copy);
}

fn main() {
    // let mut map_lines: Vec<&str> = Vec::new();
    // let mut move_lines: Vec<&str> = Vec::new();
    // let mut line: String = String::new();

    let mut buffer = String::new();
    while let Ok(read) = io::stdin().read_line(&mut buffer) {
        if read == 0 {
            break;
        }
    }
    let splits: Vec<&str> = buffer.trim_end().split("\n\n").collect();
    let (map, mut moves) = interpret_map_and_moves(
        splits[0].split("\n").collect(),
        splits[1].split("\n").collect(),
    );

    let generate_applescript: bool = std::env::args()
        .find(|arg| arg == "--applescript")
        .is_some();
    if generate_applescript {
        println!("activate application \"Golf Peaks\"");
        println!("delay 0.1");
    }
    if let Some(solution_moves) = try_moves_to_reach_hole(&map, &Location { x: 0, y: 0 }, &moves) {
        let mut last_index = 0;
        for (i, direction) in solution_moves {
            if generate_applescript {
                let next_position = i - last_index;
                for _ in 0..(next_position.abs()) {
                    if next_position > 0 {
                        println!("tell application \"System Events\" to keystroke \"e\"");
                        println!("delay 0.1");
                    } else {
                        println!("tell application \"System Events\" to keystroke \"q\"");
                        println!("delay 0.1");
                    }
                }
                println!(
                    "tell application \"System Events\" to keystroke \"{}\"",
                    match direction {
                        Direction::Up => "w",
                        Direction::Down => "s",
                        Direction::Left => "a",
                        Direction::Right => "d",
                    }
                );
                println!("delay 0.1");
                println!("tell application \"System Events\" to key code 36");
                println!("delay 2");
            } else {
                println!(
                    "Use {} {}",
                    moves[i as usize].distance,
                    match direction {
                        Direction::Up => "up",
                        Direction::Down => "down",
                        Direction::Left => "left",
                        Direction::Right => "right",
                    }
                );
            }
            moves.remove(i as usize);
            last_index = i - 1
        }
    } else {
        std::process::exit(1);
    }
}
