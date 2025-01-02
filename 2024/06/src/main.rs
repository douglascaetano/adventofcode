mod map;

use std::collections::HashSet;
use std::io::Read;

use self::map::Map;
use self::map::Position;

struct Guard {
    current: Option<Position>,
    path: Vec<Position>,
    past_positions: HashSet<Position>,
    map: Map,
}

impl Guard {
    fn new(map: Map) -> Guard {
        Guard {
            current: Some(map.guard_start_position()),
            path: vec![map.guard_start_position()],
            past_positions: HashSet::from([map.guard_start_position()]),
            map,
        }
    }

    fn r#move(&mut self) -> Option<Position> {
        if let Some(position) = self.current {
            let next_position = position.r#move();

            if !self.map.is_point_on_grid(next_position.point) {
                self.current = None;
            } else if self.map.is_point_obstructed(next_position.point) {
                self.current = Some(position.rotate_clockwise());
            } else {
                self.current = Some(next_position);
            }

            self.path.push(position);
            self.past_positions.insert(position);
        }

        self.current
    }

    fn move_until_out_of_grid(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(position) = self.r#move() {
            if self.past_positions.contains(&position) {
                return Err("loop detected".into());
            }
        }

        Ok(())
    }
}

fn count_unique_points(map: &Map) -> usize {
    let mut guard = Guard::new(map.clone());
    guard.move_until_out_of_grid().expect("map should be valid");
    guard
        .past_positions
        .iter()
        .map(|p| p.point)
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 6 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let map: Map = input.parse()?;

    println!(
        "Part 1: distinct points guard will visit: {}",
        count_unique_points(&map)
    );
    println!("Part 2: ?");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::map::Direction;
    use crate::map::Point;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_guard_initial_position() {
        let map: Map = SAMPLE.parse().unwrap();

        let guard = Guard::new(map);

        assert_eq!(
            guard.current,
            Some(Position {
                point: Point::new(6, 4),
                direction: Direction::North
            })
        );
    }

    #[test]
    fn test_guard_move() {
        let map: Map = SAMPLE.parse().unwrap();
        let mut guard = Guard::new(map);

        let new_position = guard.r#move();

        assert_eq!(
            new_position,
            Some(Position {
                point: Point::new(5, 4),
                direction: Direction::North
            })
        );
    }

    #[test]
    fn test_count_unique_points() {
        let map: Map = SAMPLE.parse().unwrap();

        let unique_points = count_unique_points(&map);

        assert_eq!(unique_points, 41);
    }
}
