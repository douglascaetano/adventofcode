mod map;

use std::collections::HashSet;
use std::io::Read;

use self::map::Map;
use self::map::Point;

fn predict_guard_path(map: &Map) -> Vec<Point> {
    let mut guard_position = map.guard_start_position();

    let mut guard_path = vec![guard_position.position];

    loop {
        let next_position = guard_position.position.move_to(guard_position.direction);

        if !map.point_is_valid(next_position) {
            break;
        }

        if map.point_is_obstructed(next_position) {
            guard_position.direction = guard_position.direction.rotate_clockwise();
            continue;
        }

        guard_position.position = next_position;
        guard_path.push(next_position);
    }

    guard_path
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 6 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let map: Map = input.parse()?;

    let guard_path = predict_guard_path(&map);

    let unique_positions: HashSet<Point> = HashSet::from_iter(guard_path);

    println!(
        "Part 1: distinct positions guard will visit: {}",
        unique_positions.len()
    );
    println!("Part 2: ?");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_predict_guard_path() {
        let map: Map = SAMPLE.parse().unwrap();

        let guard_path = predict_guard_path(&map);
        let unique_positions: HashSet<Point> = HashSet::from_iter(guard_path.clone());

        assert_eq!(guard_path[0], Point::new(6, 4));
        assert_eq!(guard_path.last(), Some(&Point::new(9, 7)));
        assert_eq!(unique_positions.len(), 41);
    }
}