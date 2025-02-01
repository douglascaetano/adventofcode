mod map;

use std::collections::HashSet;
use std::io::Read;

use self::map::Map;
use self::map::Point;
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

fn unique_points(guard: &Guard) -> HashSet<Point> {
    guard
        .past_positions
        .iter()
        .map(|p| p.point)
        .collect::<HashSet<_>>()
}

fn possible_new_obstructions(guard: &Guard) -> HashSet<Point> {
    guard
        .past_positions
        .iter()
        .map(|p| p.r#move().point)
        .filter(|p| guard.map.is_point_on_grid(*p) && !guard.map.is_point_obstructed(*p))
        .filter(|p| {
            let mut map = guard.map.clone();
            map.obstructions.insert(*p);

            let mut guard = Guard::new(map);
            guard.move_until_out_of_grid().is_err()
        })
        .collect::<HashSet<_>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 6 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let map: Map = input.parse()?;
    let mut guard = Guard::new(map);

    guard.move_until_out_of_grid()?;

    println!(
        "Part 1: distinct points guard will visit: {}",
        unique_points(&guard).len()
    );
    println!(
        "Part 2: number of possible new obstructions: {}",
        possible_new_obstructions(&guard).len()
    );

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
        let mut guard = Guard::new(map);
        guard.move_until_out_of_grid().unwrap();

        let unique_points = unique_points(&guard);

        assert_eq!(unique_points.len(), 41);
    }

    #[test]
    fn test_possible_new_obstructions() {
        let map: Map = SAMPLE.parse().unwrap();
        let mut guard = Guard::new(map);
        guard.move_until_out_of_grid().unwrap();

        let obstructions = dbg!(possible_new_obstructions(&guard));

        assert_eq!(obstructions.len(), 6);
        assert_eq!(
            obstructions,
            HashSet::from([
                Point::new(6, 3),
                Point::new(7, 6),
                Point::new(7, 7),
                Point::new(8, 1),
                Point::new(8, 3),
                Point::new(9, 7),
            ])
        );
    }
}
