use std::collections::HashMap;
use std::io;

#[derive(Clone, Copy)]
enum Corner {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

#[derive(PartialEq)]
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

#[derive(Clone, Copy)]
pub struct Move {
    distance: i32,
    airborne: i32,
}

#[derive(Clone, Copy, PartialEq)]
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
    position: Location,
    moves: Vec<Move>,
) -> Option<Vec<(i32, Direction, i32)>> {
    // check whether hole reached
    if let Some(current_tile) = map.get(&position) {
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
            if let Some((end_position, steps)) = try_move(&map, position, moves[i], *direction) {
                let mut remaining_moves = moves.clone();
                remaining_moves.remove(i);
                if let Some(mut moves_to_solve) =
                    try_moves_to_reach_hole(map, end_position, remaining_moves)
                {
                    moves_to_solve.insert(0, (i as i32, *direction, steps));
                    return Some(moves_to_solve);
                }
            }
        }
    }
    return None;
}

fn opposite_direction_of(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

// attempts to move with the nominated put/direction
// returns the finishing position, or None for moving/finishing OOB
fn try_move(
    map: &HashMap<Location, Tile>,
    starting_position: Location,
    mut remaining_move: Move,
    mut current_direction: Direction,
) -> Option<(Location, i32)> {
    let steps = 0;
    let mut last_stable_position = starting_position;
    let mut current_position = starting_position;

    while remaining_move.distance > 0 || remaining_move.airborne > 0 {
        let mut current_tile = map.get(&current_position).unwrap();
        let mut next_position = current_position;
        let airborne = remaining_move.airborne > 0;

        // IDENTIFY NEXT POSITION
        if airborne {
            match current_direction {
                Direction::Up => next_position.y += remaining_move.airborne,
                Direction::Right => next_position.x += remaining_move.airborne,
                Direction::Down => next_position.y -= remaining_move.airborne,
                Direction::Left => next_position.x -= remaining_move.airborne,
            }
        } else {
            if let Some(corner) = current_tile.corner {
                match current_direction {
                    Direction::Up => match corner {
                        Corner::Northeast => current_direction = Direction::Left,
                        Corner::Northwest => current_direction = Direction::Right,
                        _ => (),
                    },
                    Direction::Right => match corner {
                        Corner::Northeast => current_direction = Direction::Down,
                        Corner::Southeast => current_direction = Direction::Up,
                        _ => (),
                    },
                    Direction::Down => match corner {
                        Corner::Southeast => current_direction = Direction::Left,
                        Corner::Southwest => current_direction = Direction::Right,
                        _ => (),
                    },
                    Direction::Left => match corner {
                        Corner::Southwest => current_direction = Direction::Up,
                        Corner::Northwest => current_direction = Direction::Down,
                        _ => (),
                    },
                }
            }
            match current_direction {
                Direction::Up => next_position.y += 1,
                Direction::Right => next_position.x += 1,
                Direction::Down => next_position.y -= 1,
                Direction::Left => next_position.x -= 1,
            };
        }

        // Attempt to move to the next tile
        if current_tile.terrain == Terrain::Trap && !airborne {
            remaining_move.distance = 0;
        } else if let Some(next_tile) = map.get(&next_position) {
            if airborne {
                remaining_move.airborne = 0;
                current_position = next_position;
            } else {
                remaining_move.distance -= 1;
                if current_tile.elevation > next_tile.elevation {
                    current_position = next_position;
                } else if current_tile.elevation == next_tile.elevation {
                    // the next tile may have a corner that blocks rolling
                    let next_tile_has_corner: bool;
                    if let Some(corner) = next_tile.corner {
                        next_tile_has_corner = match current_direction {
                            Direction::Up => match corner {
                                Corner::Southeast => true,
                                Corner::Southwest => true,
                                _ => false,
                            },
                            Direction::Right => match corner {
                                Corner::Southwest => true,
                                Corner::Northwest => true,
                                _ => false,
                            },
                            Direction::Down => match corner {
                                Corner::Northeast => true,
                                Corner::Northwest => true,
                                _ => false,
                            },
                            Direction::Left => match corner {
                                Corner::Northeast => true,
                                Corner::Southeast => true,
                                _ => false,
                            },
                        }
                    } else {
                        next_tile_has_corner = false;
                    }
                    if next_tile_has_corner {
                        current_direction = opposite_direction_of(&current_direction);
                    } else {
                        current_position = next_position;
                    }
                } else {
                    let mut can_ascend = false;
                    if let Terrain::Slope(slope_dir) = next_tile.terrain {
                        if current_tile.elevation == next_tile.elevation - 1 {
                            can_ascend = current_direction == opposite_direction_of(&slope_dir);
                        }
                    }
                    if can_ascend {
                        current_position = next_position;
                    } else {
                        current_direction = opposite_direction_of(&current_direction);
                    }
                }
            }
        } else {
            return None;
        }

        // Apply logic depending on the tile you land on
        current_tile = map.get(&current_position).unwrap();
        if current_tile.terrain == Terrain::Hole {
            if airborne {
                return Some((current_position, steps));
            }
        } else if let Terrain::Slope(slope_dir) = current_tile.terrain {
            if airborne
                || current_direction != opposite_direction_of(&slope_dir)
                || remaining_move.distance == 0
            {
                current_direction = slope_dir;
                if remaining_move.distance == 0 {
                    remaining_move.distance += 1;
                }
            }
        } else if current_tile.terrain == Terrain::Water {
            return Some((last_stable_position, steps));
        } else if current_tile.terrain == Terrain::Spring {
            remaining_move.airborne = remaining_move.distance;
            remaining_move.distance = 0;
        } else if let Terrain::Portal(exit_portal) = current_tile.terrain {
            if airborne || remaining_move.distance == 0 {
                current_position = exit_portal;
            }
        }

        last_stable_position = match current_tile.terrain {
            Terrain::Hole => current_position,
            Terrain::Ground => current_position,
            Terrain::Slope(_) => last_stable_position,
            Terrain::Trap => current_position,
            Terrain::Quicksand => last_stable_position,
            Terrain::Water => last_stable_position,
            Terrain::Spring => current_position,
            Terrain::Portal(_) => current_position,
        }
    }

    if let Some(stopping_tile) = map.get(&current_position) {
        if stopping_tile.terrain == Terrain::Quicksand {
            return None;
        }
    }

    return Some((current_position, steps));
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
    if let Some(solution_moves) = try_moves_to_reach_hole(&map, starting_position, moves.clone()) {
        for (i, direction, _steps) in solution_moves {
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
                println!("delay 5");
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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn falls_out_of_bounds_if_rolling_across_gaps() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn skips_over_intermediate_tiles_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 0, airborne: 2 }, Direction::Right);
        
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn uses_airborne_movement_before_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 2 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }

    #[test]
    fn bounces_off_walls() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: -1, y: 0 });
    }
    
    #[test]
    fn stops_on_hole_if_landing_from_airborne_even_if_can_keep_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Hole, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 2 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn returns_finishing_position_even_if_no_net_movement() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    
    #[test]
    fn bounces_off_back_of_corner_like_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Northwest) });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn is_not_blocked_by_corner_wall_if_dropping_down() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Northwest) });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn changes_direction_when_dropping_down_onto_slope() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Up), elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 3, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn rolls_up_slope_if_facing_right_direction() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 1, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn rolls_down_slope_if_not_going_uphill() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Up), elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    
    #[test]
    fn rolls_down_slope_if_move_runs_out() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn always_rolls_down_slope_if_landing_from_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Left), elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 2 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_roll_out_of_trap() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_escape_trap_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 0, airborne: 1 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn sinks_if_stops_on_quicksand() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 3, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_ground_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_slope_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::Right), elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: -1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_quicksand_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_spring_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 3, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }
    
    #[test]
    fn does_not_get_launched_airborne_if_starting_on_spring() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }
    
    #[test]
    fn gets_launched_airborne_after_bouncing_off_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 2, airborne: 0 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn goes_through_portal_if_stops_while_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 2 }), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 0 }), elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 0, airborne: 1 }, Direction::Right);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 1 }, Direction::Right);

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

        // let result = try_move(&map, Location { x: 0, y: 0 }, Move { distance: 1, airborne: 0 }, Direction::Right);

        // assert_eq!(result.is_some(), true);
        // assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
        assert_eq!(true, false);
    }
}
