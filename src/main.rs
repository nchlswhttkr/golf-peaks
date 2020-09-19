use std::collections::{HashMap, HashSet};
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
    Conveyor(Direction),
    Ice,
}

struct Tile {
    terrain: Terrain,
    elevation: i32,
    corner: Option<Corner>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Card {
    rolling: i32,
    airborne: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Location {
    x: i32,
    y: i32,
}

fn main() {
    // Read level from STDIN
    let mut buffer = String::new();
    while let Ok(read) = io::stdin().read_line(&mut buffer) {
        if read == 0 {
            break;
        }
    }
    let splits: Vec<&str> = buffer.trim_end().split("\n\n").collect();
    let (map, mut all_cards, starting_position) = interpret_starting_conditions(
        splits[0].split("\n").collect(),
        splits[1].split("\n").collect(),
        splits[2],
    );
    let mut unique_cards: Vec<Card> = Vec::new();
    let mut card_count: Vec<i32> = Vec::new();
    for card in &all_cards {
        if let Some(i) = unique_cards.iter().position(|c| c == card) {
            card_count[i] += 1;
        } else {
            unique_cards.push(*card);
            card_count.push(1);
        }
    }

    // Determine output format (plain, applescript, step)
    let generate_applescript: bool = std::env::args()
        .find(|arg| arg == "--applescript")
        .is_some();
    let show_step_count: bool = std::env::args().find(|arg| arg == "--steps").is_some();

    // Attempt to solve, return appropriate output if a solution is found
    if let Some(solution_moves) = try_moves_to_reach_hole(
        &map,
        starting_position,
        &unique_cards,
        &mut card_count,
        &mut Vec::new(),
        &mut HashMap::new(),
        None,
    ) {
        if show_step_count {
            println!("{}", solution_moves.iter().map(|(_, _, s)| s).sum::<i32>())
        } else if generate_applescript {
            println!("activate application \"Golf Peaks\"");
            for (card, direction, steps) in solution_moves {
                let i = all_cards.iter().position(|&c| c == card).unwrap();
                if i > all_cards.len() / 2 {
                    for _ in 0..(all_cards.len() - i) {
                        println!("tell application \"System Events\" to keystroke \"q\"");
                        println!("delay 0.05");
                    }
                } else {
                    for _ in 0..i {
                        println!("tell application \"System Events\" to keystroke \"e\"");
                        println!("delay 0.05");
                    }
                }
                println!(
                    "tell application \"System Events\" to keystroke \"{}\"",
                    match direction {
                        Direction::North => "w",
                        Direction::South => "s",
                        Direction::West => "a",
                        Direction::East => "d",
                    }
                );
                println!("delay 0.05");
                println!("tell application \"System Events\" to key code 36");
                println!("delay {}", steps as f64 / 3.0);
                // FIXME timing is off on extremely long moves, add a buffer
                if steps > 18 {
                    println!("delay 0.5")
                }
                all_cards.remove(i);
            }
        } else {
            for (card, direction, _) in solution_moves {
                println!(
                    "Use {}/{} {}",
                    card.airborne,
                    card.rolling,
                    match direction {
                        Direction::North => "up",
                        Direction::South => "down",
                        Direction::West => "left",
                        Direction::East => "right",
                    }
                );
            }
        }
    } else {
        std::process::exit(1);
    }
}

fn interpret_starting_conditions(
    map_lines: Vec<&str>,
    move_lines: Vec<&str>,
    starting_position_line: &str,
) -> (HashMap<Location, Tile>, Vec<Card>, Location) {
    // Read every tile into the mail
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
                        "north" => Direction::North,
                        "south" => Direction::South,
                        "west" => Direction::West,
                        "east" => Direction::East,
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
        } else if items[0] == "conveyor" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Conveyor(match items[4] {
                        "north" => Direction::North,
                        "south" => Direction::South,
                        "west" => Direction::West,
                        "east" => Direction::East,
                        _ => panic!("Unknown conveyor direction"),
                    }),
                    elevation: items.get(3).unwrap_or(&"0").parse::<i32>().unwrap(),
                    corner: None,
                },
            );
        } else if items[0] == "ice" {
            map.insert(
                Location {
                    x: items[1].parse::<i32>().unwrap(),
                    y: items[2].parse::<i32>().unwrap(),
                },
                Tile {
                    terrain: Terrain::Ice,
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
        }
    }

    // Read every move
    let moves: Vec<Card> = move_lines
        .iter()
        .map(|m| {
            let s: Vec<&str> = m.split(",").collect();
            return Card {
                rolling: s[1].parse::<i32>().unwrap(),
                airborne: s[0].parse::<i32>().unwrap(),
            };
        })
        .collect();

    // Parse the starting positions
    let coords: Vec<&str> = starting_position_line.split(',').collect();
    let starting_position = Location {
        x: coords[0].parse::<i32>().unwrap(),
        y: coords[1].parse::<i32>().unwrap(),
    };

    return (map, moves, starting_position);
}

fn try_moves_to_reach_hole(
    map: &HashMap<Location, Tile>,
    position: Location,
    cards: &Vec<Card>,
    mut card_counts: &mut Vec<i32>,
    mut previous_positions: &mut Vec<Location>,
    mut known_moves: &mut HashMap<(Location, Card, Direction), Option<(Location, i32)>>,
    mut step_count_to_beat: Option<i32>,
) -> Option<Vec<(Card, Direction, i32)>> {
    previous_positions.push(position);
    let mut solution: Option<Vec<(Card, Direction, i32)>> = None;
    let cards_to_use: Vec<usize> = card_counts
        .iter()
        .enumerate()
        .filter_map(|(i, &count)| {
            if count > 0 {
                return Some(i);
            } else {
                return None;
            }
        })
        .collect();
    for i in cards_to_use {
        card_counts[i] -= 1;
        let current_card = cards[i];
        for direction in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        {
            let move_result;
            if let Some(known_move) = known_moves.get(&(position, current_card, *direction)) {
                move_result = *known_move;
            } else {
                move_result = try_move(&map, position, current_card, *direction);
                known_moves.insert((position, current_card, *direction), move_result);
            }
            if let Some((end_position, steps)) = move_result {
                let remaining_steps;
                if let Some(max_steps) = step_count_to_beat {
                    remaining_steps = Some(max_steps - steps);
                } else {
                    remaining_steps = None;
                }
                // Only evaluate moves that beat the target step count
                if remaining_steps.is_none() || remaining_steps.unwrap() > 0 {
                    // If movement ends on the hole it must be an acceptable solution
                    if map.get(&end_position).unwrap().terrain == Terrain::Hole {
                        solution = Some(vec![(current_card, *direction, steps)]);
                        step_count_to_beat = Some(steps);
                    // Otherwise, keep building a path to try and reach the hole
                    } else if !previous_positions.contains(&end_position) {
                        if let Some(mut moves_to_solve) = try_moves_to_reach_hole(
                            map,
                            end_position,
                            &cards,
                            &mut card_counts,
                            &mut previous_positions,
                            &mut known_moves,
                            remaining_steps,
                        ) {
                            moves_to_solve.insert(0, (current_card, *direction, steps));
                            step_count_to_beat = Some(
                                moves_to_solve
                                    .iter()
                                    .map(|(_, _, steps)| steps)
                                    .sum::<i32>(),
                            );
                            solution = Some(moves_to_solve);
                        }
                    }
                }
            }
        }
        card_counts[i] += 1;
    }
    previous_positions.pop();
    return solution;
}

fn opposite_direction_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

// attempts to move with the nominated put/direction
// returns the finishing position, or None for moving/finishing OOB
fn try_move(
    map: &HashMap<Location, Tile>,
    starting_position: Location,
    mut remaining_card: Card,
    mut current_direction: Direction,
) -> Option<(Location, i32)> {
    let mut steps = 3;
    let mut last_stable_position = starting_position;
    let mut current_position = starting_position;
    let mut infinite_loop_guard: HashSet<(Location, Direction)> = HashSet::new();

    while remaining_card.rolling > 0 || remaining_card.airborne > 0 {
        let tile_before_moving = map.get(&current_position).unwrap();
        let position_before_moving = current_position;
        let mut next_position = current_position;
        let moving_by_air = remaining_card.airborne > 0;

        // IDENTIFY NEXT POSITION
        if moving_by_air {
            match current_direction {
                Direction::North => next_position.y += remaining_card.airborne,
                Direction::East => next_position.x += remaining_card.airborne,
                Direction::South => next_position.y -= remaining_card.airborne,
                Direction::West => next_position.x -= remaining_card.airborne,
            }
        } else {
            if let Some(corner) = tile_before_moving.corner {
                match current_direction {
                    Direction::North => match corner {
                        Corner::Northeast => current_direction = Direction::West,
                        Corner::Northwest => current_direction = Direction::East,
                        _ => (),
                    },
                    Direction::East => match corner {
                        Corner::Northeast => current_direction = Direction::South,
                        Corner::Southeast => current_direction = Direction::North,
                        _ => (),
                    },
                    Direction::South => match corner {
                        Corner::Southeast => current_direction = Direction::West,
                        Corner::Southwest => current_direction = Direction::East,
                        _ => (),
                    },
                    Direction::West => match corner {
                        Corner::Southwest => current_direction = Direction::North,
                        Corner::Northwest => current_direction = Direction::South,
                        _ => (),
                    },
                }
            }
            match current_direction {
                Direction::North => next_position.y += 1,
                Direction::East => next_position.x += 1,
                Direction::South => next_position.y -= 1,
                Direction::West => next_position.x -= 1,
            };
        }

        // Attempt to move to the next tile
        if tile_before_moving.terrain == Terrain::Trap && !moving_by_air {
            remaining_card.rolling = 0;
        } else if let Some(next_tile) = map.get(&next_position) {
            if moving_by_air {
                steps += remaining_card.airborne;
                remaining_card.airborne = 0;
                current_position = next_position;
            } else {
                steps += 1;
                remaining_card.rolling -= 1;
                if tile_before_moving.elevation > next_tile.elevation {
                    // Go to next tile always if it is lower
                    current_position = next_position;
                } else if tile_before_moving.elevation == next_tile.elevation {
                    // Check for the back of a corner blocking the next tile
                    let next_tile_has_corner: bool;
                    if let Some(corner) = next_tile.corner {
                        next_tile_has_corner = match current_direction {
                            Direction::North => match corner {
                                Corner::Southeast => true,
                                Corner::Southwest => true,
                                _ => false,
                            },
                            Direction::East => match corner {
                                Corner::Southwest => true,
                                Corner::Northwest => true,
                                _ => false,
                            },
                            Direction::South => match corner {
                                Corner::Northeast => true,
                                Corner::Northwest => true,
                                _ => false,
                            },
                            Direction::West => match corner {
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
                    // Rolling balls can only "ascend" if they up a slope
                    let mut can_ascend = false;
                    if let Terrain::Slope(slope_dir) = next_tile.terrain {
                        if tile_before_moving.elevation == next_tile.elevation - 1 {
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

        // Loops only occur if the ball is "stuttering" on ice/slopes/conveyors
        if remaining_card.rolling == 0 {
            if infinite_loop_guard.contains(&(current_position, current_direction)) {
                return None;
            } else {
                infinite_loop_guard.insert((current_position, current_direction));
            }
        }

        // Apply logic depending on the tile you land on
        let landed_tile = map.get(&current_position).unwrap();
        if landed_tile.terrain == Terrain::Hole {
            // Stop if you land in the hole from the air
            if moving_by_air {
                return Some((current_position, steps));
            }
        } else if let Terrain::Slope(slope_dir) = landed_tile.terrain {
            // Turn down a slope if you are not _rolling_ directly up it
            if moving_by_air
                || current_direction != opposite_direction_of(&slope_dir)
                || remaining_card.rolling == 0
            {
                current_direction = slope_dir;
                // Ball cannot stop on a slope, keep rolling down the slope
                if remaining_card.rolling == 0 {
                    remaining_card.rolling += 1;
                }
            }
        } else if landed_tile.terrain == Terrain::Water {
            // Stop immediately upon landing in water
            steps += 3;
            return Some((last_stable_position, steps));
        } else if landed_tile.terrain == Terrain::Spring {
            // Convert rolling energy into airborne energy
            remaining_card.airborne = remaining_card.rolling;
            remaining_card.rolling = 0;
            if remaining_card.airborne == 0 {
                steps += 1; // Stopping on a spring adds a slight delay
            }
        } else if let Terrain::Portal(exit_portal) = landed_tile.terrain {
            // Fall through portal if landing (from air) or stopping on it
            if moving_by_air || remaining_card.rolling == 0 {
                steps += 1;
                current_position = exit_portal;
            }
        } else if let Terrain::Conveyor(conveyor_direction) = landed_tile.terrain {
            // Follow conveyor belt if not rolling
            if remaining_card.rolling == 0 {
                current_direction = conveyor_direction;
                remaining_card.rolling += 1;
            }
        } else if landed_tile.terrain == Terrain::Ice {
            if remaining_card.rolling == 0 && current_position != position_before_moving {
                remaining_card.rolling += 1;
            }
        }

        // Not all tiles count as stable ground (from falling into water)
        last_stable_position = match landed_tile.terrain {
            Terrain::Hole => current_position,
            Terrain::Ground => current_position,
            Terrain::Slope(_) => last_stable_position,
            Terrain::Trap => current_position,
            Terrain::Quicksand => last_stable_position,
            Terrain::Water => last_stable_position,
            Terrain::Spring => current_position,
            Terrain::Portal(_) => current_position,
            Terrain::Conveyor(_) => last_stable_position,
            Terrain::Ice => current_position,
        }
    }

    // Fail the move if it ends on quicksand
    if let Some(stopping_tile) = map.get(&current_position) {
        if stopping_tile.terrain == Terrain::Quicksand {
            return None;
        }
    }

    return Some((current_position, steps));
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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn falls_out_of_bounds_if_rolling_across_gaps() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn skips_over_intermediate_tiles_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 0, airborne: 2 }, Direction::East);
        
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn uses_airborne_movement_before_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 2 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }

    #[test]
    fn bounces_off_walls() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: -1, y: 0 });
    }
    
    #[test]
    fn stops_on_hole_if_landing_from_airborne_even_if_can_keep_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Hole, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 2 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn returns_finishing_position_even_if_no_net_movement() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    
    #[test]
    fn bounces_off_back_of_corner_like_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Northwest) });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn is_not_blocked_by_corner_wall_if_dropping_down() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: Some(Corner::Northwest) });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

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
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::East), elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn changes_direction_when_dropping_down_onto_slope() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::North), elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 3, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn rolls_up_slope_if_facing_right_direction() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::West), elevation: 1, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn rolls_down_slope_if_not_going_uphill() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::North), elevation: 1, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }
    
    #[test]
    fn rolls_down_slope_if_move_runs_out() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Slope(Direction::West), elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn always_rolls_down_slope_if_landing_from_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Slope(Direction::West), elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 2 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_roll_out_of_trap() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_escape_trap_if_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Trap, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 0, airborne: 1 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn sinks_if_stops_on_quicksand() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 3, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_ground_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_slope_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::East), elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: -1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn does_not_get_placed_back_on_quicksand_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Quicksand, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }

    #[test]
    fn gets_placed_back_on_spring_if_lands_in_water() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Water, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 3, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 3, y: 0 });
    }
    
    #[test]
    fn does_not_get_launched_airborne_if_starting_on_spring() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
    }
    
    #[test]
    fn gets_launched_airborne_after_bouncing_off_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: -1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Spring, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 2 });
    }

    #[test]
    fn goes_through_portal_if_stops_while_airborne() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 2 }), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 2 }, Tile { terrain: Terrain::Portal(Location { x: 1, y: 0 }), elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 0, airborne: 1 }, Direction::East);

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

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 1 }, Direction::East);

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
        // map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Slope(Direction::West), elevation: 0, corner: None });

        // let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        // assert_eq!(result.is_some(), true);
        // assert_eq!(result.unwrap().0, Location { x: 0, y: 0 });
        assert_eq!(true, false);
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_conveyors {
    use super::*;

    #[test]
    fn skips_over_conveyor_belts_if_rolling() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Conveyor(Direction::South), elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 2, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn follows_conveyor_belts_if_stops() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Conveyor(Direction::North), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }

    #[test]
    fn fails_if_gets_stuck_in_loop_on_conveyor() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Conveyor(Direction::North), elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_none(), true);
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test_ice {
    use super::*;

    #[test]
    fn keeps_moving_on_ice_if_stops() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn stops_moving_on_ice_if_hits_wall() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 0 });
    }

    #[test]
    fn does_not_move_on_ice_if_hits_wall_when_stopping() {
        /*
        Very similar to the above case, but only occurs when the ball runs into
        a wall on its last step. Even though it is on ice, it shouldn't start
        rolling back in the direction it came.
        */
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: None });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: None });
        map.insert(Location { x: 3, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 1, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 3, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 2, y: 0 });
    }

    #[test]
    fn bounces_off_corners_while_on_ice() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: Some(Corner::Southeast) });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 1, airborne: 0 }, Direction::East);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().0, Location { x: 1, y: 1 });
    }

    #[test]
    fn fails_if_gets_stuck_in_loop_of_ice_corners() {
        let mut map: HashMap<Location, Tile> = HashMap::new();
        map.insert(Location { x: 0, y: 0 }, Tile { terrain: Terrain::Ground, elevation: 0, corner: None });
        map.insert(Location { x: 1, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: Some(Corner::Southwest) });
        map.insert(Location { x: 2, y: 0 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: Some(Corner::Southeast) });
        map.insert(Location { x: 1, y: 1 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: Some(Corner::Northwest) });
        map.insert(Location { x: 2, y: 1 }, Tile { terrain: Terrain::Ice, elevation: 0, corner: Some(Corner::Northeast) });

        let result = try_move(&map, Location { x: 0, y: 0 }, Card { rolling: 0, airborne: 1 }, Direction::East);

        assert_eq!(result.is_none(), true);
    }
}
