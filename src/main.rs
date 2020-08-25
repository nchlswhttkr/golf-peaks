use std::collections::HashMap;
use std::io;

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
    Spring,
    Portal(Location),
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
        } else if items[0] == "spring" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Spring,
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
                },
            );
        } else if items[0] == "portal" {
            let exit = Location {
                x: items[4].parse::<i32>().unwrap(),
                y: items[5].parse::<i32>().unwrap(),
            };
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Portal(exit),
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
) -> Option<Vec<(i32, Direction, i32)>> {
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
            if let Some((end_position, steps)) = try_move(&map, position, &moves[i], &direction) {
                // println!(
                //     "trying {}/{} from {},{} {}",
                //     &moves[i].airborne,
                //     &moves[i].distance,
                //     position.x,
                //     position.y,
                //     match &direction {
                //         Direction::Up => "up",
                //         Direction::Down => "down",
                //         Direction::Left => "left",
                //         Direction::Right => "right",
                //     }
                // );
                let mut remaining_moves = moves.clone();
                remaining_moves.remove(i);
                if let Some(mut moves_to_solve) =
                    try_moves_to_reach_hole(&map, &end_position, &remaining_moves)
                {
                    moves_to_solve.insert(0, (i as i32, direction.clone(), steps));
                    return Some(moves_to_solve);
                }
            }
            // else {
            //     println!();
            // }
        }
    }
    return None;
}

// attempts to move with the nominated put/direction
// returns the finishing position, or None for moving/finishing OOB
fn try_move(
    map: &HashMap<Location, Tile>,
    position: &Location,
    chosen_move: &Move,
    chosen_direction: &Direction,
) -> Option<(Location, i32)> {
    let mut steps = 0;
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
            Terrain::Spring => true,
            Terrain::Portal(_) => true,
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
            steps += move_copy.airborne;
            move_copy.airborne = 0;
            if let Some(landing_tile) = map.get(&next_position) {
                match landing_tile.terrain {
                    // face direction of landing slope
                    Terrain::Slope(d) => direction = d,
                    // only move through portal if you'll continue out of it
                    // a later condition handles stopping on a portal (sorry!)
                    Terrain::Portal(exit) => {
                        if move_copy.distance > 0 {
                            next_position = exit
                        }
                    }
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
            steps += 1;
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

        // landed on sand or spring, apply effect
        if let Some(landed_on) = map.get(&position_copy) {
            match landed_on.terrain {
                Terrain::Trap => move_copy.distance = 0,
                Terrain::Spring => {
                    move_copy.airborne = move_copy.distance;
                    move_copy.distance = 0;
                }
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
                Terrain::Water => true,
                Terrain::Spring => true,
                Terrain::Portal(_) => true,
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

        // travel to partner if ending on portal
        if stopped {
            match cur_tile.unwrap().terrain {
                Terrain::Portal(exit) => position_copy = exit,
                _ => (),
            };
        }

        // if we land on water at any step, return the last position
        if match cur_tile.unwrap().terrain {
            Terrain::Water => true,
            _ => false,
        } {
            if last_stable_position == position_copy {
                return None;
            } else {
                return Some((last_stable_position, steps));
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
                Terrain::Spring => true,
                Terrain::Portal(_) => true,
            } {
                last_stable_position = position_copy
            }
        }
    }

    return Some((position_copy, steps));
}

fn main() {
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
        println!("delay 0.05")
    }
    if let Some(solution_moves) = try_moves_to_reach_hole(&map, &starting_position, &moves) {
        for (i, direction, steps) in solution_moves {
            if generate_applescript {
                for _ in 0..i {
                    println!("tell application \"System Events\" to keystroke \"e\"");
                    println!("delay 0.05");
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
                println!("delay 0.05");
                println!("tell application \"System Events\" to key code 36");
                println!("delay {}", (steps + 4) / 2);
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

#[cfg(test)]
#[rustfmt::skip]
mod test_general_movement {
    use super::*;

    #[test]
    fn rolls_along_ground() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn falls_out_of_bounds_if_rolling_across_gaps() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn skips_over_intermediate_tiles_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 0, airborne: 2 }, &Direction::Right);
        
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn uses_airborne_movement_before_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 2 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }

    #[test]
    fn bounces_off_walls() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: -1, y: 0 });
    }
    
    #[test]
    fn stops_on_hole_if_landing_from_airborne_even_if_can_keep_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Hole, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 2 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn returns_finishing_position_even_if_no_net_movement() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_corners {
    use super::*;

    #[test]
    fn is_redicted_if_hit_corner() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Southeast) });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    #[test]
    fn bounces_off_back_of_corner_like_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Northwest) });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_slopes {
    use super::*;

    #[test]
    fn bounces_off_slopes_higher_than_current_tile() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Right), elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn rolls_up_slope_if_facing_right_direction() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 1, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn rolls_down_slope_if_not_going_uphill() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Up), elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    
    #[test]
    fn rolls_down_slope_if_move_runs_out() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn always_rolls_down_slope_if_landing_from_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 2 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_traps {
    use super::*;

    #[test]
    fn stops_if_lands_in_trap() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 1 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_roll_out_of_trap() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_escape_trap_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 0, airborne: 1 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_quicksand {
    use super::*;

    #[test]
    fn rolls_over_quicksand() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn sinks_if_stops_on_quicksand() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_none(), true);
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_water {
    use super::*;

    #[test]
    fn does_not_keep_rolling_after_landing_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 3, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_ground_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_slope_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Right), elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: -1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_quicksand_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_spring_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_spring {
    use super::*;
    
    #[test]
    fn gets_launched_airborne_if_rolls_over_spring() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 3, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }
    
    #[test]
    fn does_not_get_launched_airborne_if_starting_on_spring() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }
    
    #[test]
    fn gets_launched_airborne_after_bouncing_off_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 2, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: -1, y: 0 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_portals {
    use super::*;

    #[test]
    fn goes_through_portal_if_stops_while_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 2 }), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 0 }), elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn goes_through_portal_if_stops_while_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 2 }), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 0 }), elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 0, airborne: 1 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn continues_rolling_out_of_portal_exit_if_lands_from_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 2 }), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 0 }), elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 2 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 1 }, &Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 2 });
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_todos_and_undefined_behaviour {
    /*
    There's a few cases of subtle behaviour that needs fixing in my solver, but
    they aren't urgent because they haven't appeared in game so far.

    Similarly, some situations have undefined behaviour that I can't be sure
    about because I haven't encountered it yet.
    */

    #[allow(unused_imports)]
    use super::*;

    #[test] #[ignore]
    fn might_go_uphill_if_rolls_off_edge_onto_lower_slope() {
        assert_eq!(true, false);
    }

    #[test] #[ignore]
    fn might_fall_back_through_portal_if_rolls_in_water_after_exiting() {
        assert_eq!(true, false);
    }

    #[test] #[ignore]
    fn fix_elevation_check_when_going_down_slope_on_same_level_as_next_tile() {
        /*
        A slope's elevation is defined by the *top* of the slope. Since a ball
        can usually go between tiles of the same elevation, it can leave the
        slope even if the bottom of said slope is technically below the
        neighbouring tile.

        It should instead bounce off the wall and start heading back up the
        slope. It might get stuck in an inifite loop if it has no rolling
        movement remaining.
        */

        // let mut map: HashMap<Location, Tile> = HashMap::new();
        // map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        // map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 0, corner: None });

        // let result = try_move(&map, &Location { x: 0, y: 0 }, &Move { distance: 1, airborne: 0 }, &Direction::Right);

        // assert_eq!(result.is_some(), true);
        // assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
        assert_eq!(true, false);
    }
}
