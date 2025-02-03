use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    line: isize,
    column: isize,
}

impl Point {
    pub fn new(line: isize, column: isize) -> Self {
        Point { line, column }
    }

    pub fn next_towards(&self, direction: Direction) -> Point {
        match direction {
            Direction::North => Point {
                line: self.line - 1,
                column: self.column,
            },
            Direction::South => Point {
                line: self.line + 1,
                column: self.column,
            },
            Direction::East => Point {
                line: self.line,
                column: self.column + 1,
            },
            Direction::West => Point {
                line: self.line,
                column: self.column - 1,
            },
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from((line, column): (isize, isize)) -> Self {
        Point::new(line, column)
    }
}

impl From<(usize, usize)> for Point {
    fn from((line, column): (usize, usize)) -> Self {
        Point::new(line as isize, column as isize)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '^' => Ok(Direction::North),
            'v' => Ok(Direction::South),
            '>' => Ok(Direction::East),
            '<' => Ok(Direction::West),
            _ => Err("Invalid direction".into()),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    pub point: Point,
    pub direction: Direction,
}

impl Position {
    pub fn next(&self) -> Self {
        Position {
            point: self.point.next_towards(self.direction),
            direction: self.direction,
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        Position {
            point: self.point,
            direction: self.direction.rotate_clockwise(),
        }
    }
}

impl PartialEq<Point> for Position {
    fn eq(&self, other: &Point) -> bool {
        self.point == *other
    }
}

#[derive(Debug, Default, Clone)]
pub struct Map {
    pub obstructions: HashSet<Point>,
    pub guard_start: Position,
    pub height: usize,
    pub width: usize,
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obstructions = HashSet::new();
        let mut guard_start = Position::default();
        let mut height = 0;
        let mut width = 0;

        for (line, line_elems) in s.lines().enumerate() {
            height += 1;

            let mut line_width = 0;

            for (column, elem) in line_elems.chars().enumerate() {
                line_width += 1;

                if elem == '#' {
                    obstructions.insert(Point::from((line, column)));
                } else if let Ok(direction) = elem.try_into() {
                    guard_start = Position {
                        point: Point::from((line, column)),
                        direction,
                    }
                }
            }

            if width == 0 {
                width = line_width;
            } else if width != line_width {
                return Err("Map is not rectangular".into());
            }
        }

        Ok(Map {
            obstructions,
            guard_start,
            height,
            width,
        })
    }
}

impl Map {
    pub fn guard_start_position(&self) -> Position {
        self.guard_start
    }

    pub fn is_point_obstructed(&self, point: Point) -> bool {
        self.obstructions.contains(&point)
    }

    pub fn is_point_on_grid(&self, point: Point) -> bool {
        (0..self.height as isize).contains(&point.line)
            && (0..self.width as isize).contains(&point.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_move_to() {
        let point = Point::new(0, 0);
        assert_eq!(point.next_towards(Direction::North), Point::new(-1, 0));
        assert_eq!(point.next_towards(Direction::South), Point::new(1, 0));
        assert_eq!(point.next_towards(Direction::East), Point::new(0, 1));
        assert_eq!(point.next_towards(Direction::West), Point::new(0, -1));
    }

    #[test]
    fn test_direction_rotate_clockwise() {
        assert_eq!(Direction::North.rotate_clockwise(), Direction::East);
        assert_eq!(Direction::East.rotate_clockwise(), Direction::South);
        assert_eq!(Direction::South.rotate_clockwise(), Direction::West);
        assert_eq!(Direction::West.rotate_clockwise(), Direction::North);
    }

    #[test]
    fn test_char_to_direction() {
        assert_eq!(Direction::try_from('^').unwrap(), Direction::North);
        assert_eq!(Direction::try_from('v').unwrap(), Direction::South);
        assert_eq!(Direction::try_from('>').unwrap(), Direction::East);
        assert_eq!(Direction::try_from('<').unwrap(), Direction::West);
    }

    #[test]
    fn test_invalid_char_to_direction() {
        assert!(Direction::try_from('#').is_err());
        assert!(Direction::try_from('.').is_err());
    }

    #[test]
    fn test_parse_map() {
        let map = ".#v\n\
                   #..\n\
                   ..#\n";

        let map: Map = map.parse().unwrap();

        assert_eq!(map.height, 3);
        assert_eq!(map.width, 3);
        assert_eq!(map.guard_start.point.line, 0);
        assert_eq!(map.guard_start.point.column, 2);
        assert_eq!(map.guard_start.direction, Direction::South);
        assert_eq!(map.obstructions.len(), 3);
        assert_eq!(
            map.obstructions,
            HashSet::from([Point::new(0, 1), Point::new(1, 0), Point::new(2, 2)])
        );
        assert!(map.is_point_obstructed(Point::new(0, 1)));
        assert!(map.is_point_on_grid(Point::new(0, 1)));
        assert!(!map.is_point_on_grid(Point::new(3, 3)));
        assert!(!map.is_point_on_grid(Point::new(-1, -1)));
    }
}
