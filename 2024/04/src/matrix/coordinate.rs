use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from((row, col): (usize, usize)) -> Self {
        Coordinate::new(row, col)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    pub row_delta: isize,
    pub col_delta: isize,
}

impl Distance {
    pub fn new(row_delta: isize, col_delta: isize) -> Self {
        Distance {
            row_delta,
            col_delta,
        }
    }
}

impl From<(isize, isize)> for Distance {
    fn from((row_delta, col_delta): (isize, isize)) -> Self {
        Distance::new(row_delta, col_delta)
    }
}

impl Add<Distance> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Distance) -> Self::Output {
        Coordinate::new(
            (self.row as isize + rhs.row_delta) as usize,
            (self.col as isize + rhs.col_delta) as usize,
        )
    }
}

impl Add<(isize, isize)> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        self + Distance::from(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_from_tuple() {
        let coordinate = Coordinate::from((1, 2));
        assert_eq!(1, coordinate.row);
        assert_eq!(2, coordinate.col);
    }

    #[test]
    fn test_distance_from_tuple() {
        let distance = Distance::from((1, 2));
        assert_eq!(1, distance.row_delta);
        assert_eq!(2, distance.col_delta);
    }

    #[test]
    fn test_coordinate_add_distance() {
        let coordinate = Coordinate::from((1, 2));
        let distance = Distance::from((3, 4));
        assert_eq!(Coordinate::from((4, 6)), coordinate + distance);
    }

    #[test]
    fn test_coordinate_add_tuple() {
        let coordinate = Coordinate::from((1, 2));
        assert_eq!(Coordinate::from((2, 3)), coordinate + (1, 1));
    }
}
