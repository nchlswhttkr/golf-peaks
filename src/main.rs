use std::collections::HashMap;
use std::io;

// struct Portal {
//     exit: Location,
// }

#[derive(Clone, Copy)]
enum Corner {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

enum Terrain {
    Hole,
    Ground,
    Slope(Direction),
    Trap,
    Quicksand,
    Water,
    // Spring,
    // Portal,
}

struct Tile {
    terrain: Terrain,
    elevation: i32,
    corner: Option<Corner>,
}

#[derive(Clone)]
pub struct Move {
    distance: i32,
    airborne: i32,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
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
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: match items.get(4).unwrap_or(&"") {
                        &"nw" => Some(Corner::Northwest),
                        &"ne" => Some(Corner::Northeast),
                        &"se" => Some(Corner::Southeast),
                        &"sw" => Some(Corner::Southwest),
                        _ => None,
                    },
                },
            );
        } else if items[0] == "slope" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Slope(match items[4] {
                        "up" => Direction::Up,
                        "down" => Direction::Down,
                        "left" => Direction::Left,
                        "right" => Direction::Right,
                        _ => panic!("Unknown slope direction"),
                    }),
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
                },
            );
        } else if items[0] == "trap" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Trap,
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
                },
            );
        } else if items[0] == "sand" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Quicksand,
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
                },
            );
        } else if items[0] == "water" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Water,
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
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
                airborne: s.get(1).unwrap_or(&"0").parse::<i32>().unwrap(),
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
    let mut last_stable_position = position.clone();
    let mut direction = chosen_direction.clone();
    let mut cur_tile = map.get(&position_copy);
    let mut in_bounds = cur_tile.is_some();
    let mut stopped = move_copy.distance <= 0
        && move_copy.airborne <= 0
        && match cur_tile.unwrap().terrain {
            Terrain::Hole => true,
            Terrain::Ground => true,
            Terrain::Slope(_) => false,
            Terrain::Trap => true,
            Terrain::Quicksand => true,
            Terrain::Water => true,
        };
    while in_bounds && !stopped {
        if move_copy.airborne > 0 {
            let mut next_position = position_copy.clone();
            match direction {
                Direction::Up => next_position.y += move_copy.airborne,
                Direction::Down => next_position.y -= move_copy.airborne,
                Direction::Left => next_position.x -= move_copy.airborne,
                Direction::Right => next_position.x += move_copy.airborne,
            };
            move_copy.airborne = 0;
            if let Some(landing_tile) = map.get(&next_position) {
                match landing_tile.terrain {
                    // face direction of landing slope
                    Terrain::Slope(d) => direction = d,
                    _ => (),
                }
                position_copy = next_position;
            } else {
                return None;
            }
        } else {
            // identify potential next position
            let mut next_position = position_copy.clone();
            if let Some(corner) = cur_tile.unwrap().corner {
                match corner {
                    Corner::Northwest => match direction {
                        Direction::Up => direction = Direction::Right,
                        Direction::Left => direction = Direction::Down,
                        _ => (),
                    },
                    Corner::Northeast => match direction {
                        Direction::Up => direction = Direction::Left,
                        Direction::Right => direction = Direction::Down,
                        _ => (),
                    },
                    Corner::Southeast => match direction {
                        Direction::Down => direction = Direction::Left,
                        Direction::Right => direction = Direction::Up,
                        _ => (),
                    },
                    Corner::Southwest => match direction {
                        Direction::Down => direction = Direction::Right,
                        Direction::Left => direction = Direction::Up,
                        _ => (),
                    },
                }
            }
            match direction {
                Direction::Up => next_position.y += 1,
                Direction::Down => next_position.y -= 1,
                Direction::Left => next_position.x -= 1,
                Direction::Right => next_position.x += 1,
            }
            // move
            move_copy.distance -= 1;
            if let Some(next_tile) = map.get(&next_position) {
                // you can run into the back of a corner
                let will_hit_corner = cur_tile.unwrap().elevation == next_tile.elevation
                    && next_tile.corner.is_some()
                    && match next_tile.corner.unwrap() {
                        Corner::Northwest => match direction {
                            Direction::Down => true,
                            Direction::Right => true,
                            _ => false,
                        },
                        Corner::Northeast => match direction {
                            Direction::Down => true,
                            Direction::Left => true,
                            _ => false,
                        },
                        Corner::Southeast => match direction {
                            Direction::Up => true,
                            Direction::Left => true,
                            _ => false,
                        },
                        Corner::Southwest => match direction {
                            Direction::Up => true,
                            Direction::Right => true,
                            _ => false,
                        },
                    };
                let caught_in_trap = match cur_tile.unwrap().terrain {
                    Terrain::Trap => true,
                    _ => false,
                };
                if caught_in_trap {
                    // do not move
                } else if cur_tile.unwrap().elevation >= next_tile.elevation && !will_hit_corner {
                    position_copy = next_position;
                    if let Some(slope_dir) = match next_tile.terrain {
                        Terrain::Slope(d) => Some(d),
                        _ => None,
                    } {
                        direction = slope_dir;
                    }
                } else if cur_tile.unwrap().elevation == next_tile.elevation - 1
                && move_copy.distance > 0 //has not stopped
                && match next_tile.terrain {
                    Terrain::Slope(slope_dir) => match slope_dir {
                        Direction::Up => direction == Direction::Down,
                        Direction::Down => direction == Direction::Up,
                        Direction::Left => direction == Direction::Right,
                        Direction::Right => direction == Direction::Left,
                    },
                    _ => false,
                } {
                    position_copy = next_position;
                } else {
                    // turn around
                    direction = match direction {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                    }
                }
            } else {
                return None; // move falls out of bounds
            }
        }

        cur_tile = map.get(&position_copy);

        // landed on sand
        if let Some(landed_on) = map.get(&position_copy) {
            match landed_on.terrain {
                Terrain::Trap => move_copy.distance = 0,
                _ => (),
            };
        }

        in_bounds = cur_tile.is_some();
        stopped = move_copy.distance <= 0
            && move_copy.airborne <= 0
            && match cur_tile.unwrap().terrain {
                Terrain::Hole => true,
                Terrain::Ground => true,
                Terrain::Slope(_) => false,
                Terrain::Trap => true,
                Terrain::Quicksand => true,
                Terrain::Water => false,
            };

        // sink in quicksand if stopped
        if stopped
            && match cur_tile.unwrap().terrain {
                Terrain::Quicksand => true,
                _ => false,
            }
        {
            return None; // this move fails
        }

        // if we land on water at any step, return the last position
        if match cur_tile.unwrap().terrain {
            Terrain::Water => true,
            _ => false,
        } {
            if last_stable_position == position_copy {
                return None;
            } else {
                return Some(last_stable_position);
            }
        } else {
            // update last stable if we're on "safe" ground
            if match cur_tile.unwrap().terrain {
                Terrain::Hole => true,
                Terrain::Ground => true,
                Terrain::Trap => true,
                Terrain::Slope(_) => false,
                Terrain::Water => false,
                Terrain::Quicksand => false,
            } {
                last_stable_position = position_copy
            }
        }
    }
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
    let mut starting_position = Location { x: 0, y: 0 };
    if let Some(included_starting_position) = splits.get(2) {
        let coords: Vec<&str> = included_starting_position.split(',').collect();
        starting_position = Location {
            x: coords[0].parse::<i32>().unwrap(),
            y: coords[1].parse::<i32>().unwrap(),
        }
    }

    let generate_applescript: bool = std::env::args()
        .find(|arg| arg == "--applescript")
        .is_some();
    if generate_applescript {
        println!("activate application \"Golf Peaks\"");
        println!("delay 0.1")
    }
    if let Some(solution_moves) = try_moves_to_reach_hole(&map, &starting_position, &moves) {
        for (i, direction) in solution_moves {
            if generate_applescript {
                for _ in 0..i {
                    println!("tell application \"System Events\" to keystroke \"e\"");
                    println!("delay 0.1");
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
                println!("delay 3.5");
            } else {
                println!(
                    "Use {}/{} {}",
                    moves[i as usize].airborne,
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
        }
    } else {
        std::process::exit(1);
    }
}
