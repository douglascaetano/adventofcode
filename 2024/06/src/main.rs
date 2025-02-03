mod map;

use std::collections::HashSet;
use std::fmt::Display;
use std::io::Read;

use map::Direction;

use self::map::Map;
use self::map::Point;
use self::map::Position;

#[derive(Debug, Clone)]
struct Guard {
    current_position: Option<Position>,
    previous_positions: HashSet<Position>,
    previous_points: HashSet<Point>,
    map: Map,
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for point in (0..self.map.width).flat_map(|line| {
            (0..self.map.height).map(move |column| Point::new(line as isize, column as isize))
        }) {
            // Guard position
            if let Some(position) = self
                .current_position
                .iter()
                .find(|&&position| position.point == point)
            {
                write!(f, "{}", position.direction)?;
            }
            // Obstructions
            else if self.map.is_point_obstructed(point) {
                write!(f, "#")?;
            }
            // Starting position
            else if self.map.guard_start_position() == point {
                write!(f, "G")?;
            }
            // Past positions
            else if let Some(position) = self
                .previous_positions
                .iter()
                .find(|&&position| position == point)
            {
                write!(
                    f,
                    "{}",
                    match position.direction {
                        Direction::North | Direction::South => '|',
                        Direction::East | Direction::West => '-',
                    }
                )?;
            }
            // Empty positions
            else {
                write!(f, ".")?;
            }

            // new line
            if point.column == self.map.width as isize - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum GuardError {
    LoopDetected,
}

impl Guard {
    fn new(map: Map) -> Guard {
        Guard {
            current_position: Some(map.guard_start_position()),
            previous_positions: HashSet::new(),
            previous_points: HashSet::new(),
            map,
        }
    }

    fn r#move(&mut self) -> Option<Position> {
        if let Some(position) = self.current_position {
            self.previous_positions.insert(position);
            self.previous_points.insert(position.point);
            self.current_position = {
                let next_position = position.next();

                if !self.map.is_point_on_grid(next_position.point) {
                    None
                } else if self.map.is_point_obstructed(next_position.point) {
                    Some(position.rotate_clockwise())
                } else {
                    Some(next_position)
                }
            }
        }

        self.current_position
    }

    fn move_until_out_of_grid(&mut self) -> Result<(), GuardError> {
        while let Some(position) = self.r#move() {
            if self.previous_positions.contains(&position) {
                return Err(GuardError::LoopDetected);
            }
        }

        Ok(())
    }
}

fn unique_points(map: Map) -> HashSet<Point> {
    let mut guard = Guard::new(map);
    let _ = guard.move_until_out_of_grid();

    guard
        .previous_positions
        .iter()
        .map(|p| p.point)
        .collect::<HashSet<_>>()
}

fn possible_new_obstructions(map: Map) -> HashSet<Point> {
    let mut guard = Guard::new(map);
    let mut possible_new_obstructions = HashSet::new();

    while let Some(position) = guard.current_position {
        let next_point = position.next().point;

        if !guard.map.is_point_on_grid(next_point) {
            break;
        }

        if !guard.map.is_point_obstructed(next_point)
            && !guard.previous_points.contains(&next_point)
        {
            let mut second_guard = guard.clone();
            second_guard.map.obstructions.insert(next_point);
            if let Err(GuardError::LoopDetected) = second_guard.move_until_out_of_grid() {
                possible_new_obstructions.insert(next_point);
            }
        }

        guard.r#move();
    }

    possible_new_obstructions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 6 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let map: Map = input.parse()?;

    let unique_points = unique_points(map.clone());
    let new_obstructions = possible_new_obstructions(map.clone());

    println!(
        "Part 1: distinct points guard will visit: {}",
        unique_points.len()
    );
    println!(
        "Part 2: number of possible new obstructions: {}",
        new_obstructions.len()
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
        let initial_position = Position {
            point: Point::new(6, 4),
            direction: Direction::North,
        };

        let guard = Guard::new(map);

        assert_eq!(guard.current_position, Some(initial_position));
        assert_eq!(guard.previous_points.len(), 0);
        assert_eq!(guard.previous_positions.len(), 0);
    }

    #[test]
    fn test_guard_move() {
        let map: Map = SAMPLE.parse().unwrap();
        let mut guard = Guard::new(map);
        let expected_position = Position {
            point: Point::new(5, 4),
            direction: Direction::North,
        };
        let previous_position = guard.current_position.unwrap();

        let new_position = guard.r#move().unwrap();

        assert_eq!(new_position, expected_position);
        assert_eq!(guard.previous_points.len(), 1);
        assert!(guard.previous_points.contains(&previous_position.point));
        assert_eq!(guard.previous_positions.len(), 1);
        assert!(guard.previous_positions.contains(&previous_position));
    }

    #[test]
    fn test_count_unique_points() {
        let map: Map = SAMPLE.parse().unwrap();

        let unique_points = unique_points(map);

        assert_eq!(unique_points.len(), 41);
    }

    #[test]
    fn test_possible_new_obstructions() {
        let map: Map = SAMPLE.parse().unwrap();

        let obstructions = dbg!(possible_new_obstructions(map));

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
