mod matrix;

use std::io::Read;

use self::matrix::Coordinate;
use self::matrix::Distance;
use self::matrix::Matrix;
use self::matrix::MatrixError;

fn read_matrix(input: &str) -> Result<Matrix<char>, MatrixError> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
        .try_into()
}

fn word_is_present(
    matrix: &Matrix<char>,
    from_position: Coordinate,
    offset: Distance,
    word: &[char],
) -> bool {
    let Some(&char_to_compare) = word.get(0) else {
        return true;
    };

    let position = from_position + offset;

    matrix
        .get(position)
        .is_some_and(|&current| current == char_to_compare)
        && word_is_present(matrix, position, offset, &word[1..])
}

fn count_xmas(matrix: &Matrix<char>) -> usize {
    matrix
        .iter_enumerate()
        .filter(|(_, &elem)| elem == 'X')
        .map(|(from_position, _)| {
            (-1..=1)
                .flat_map(|row_offset| {
                    (-1..=1).map(move |col_offset| Distance::new(row_offset, col_offset))
                })
                .filter(|&offset| word_is_present(matrix, from_position, offset, &['M', 'A', 'S']))
                .count()
        })
        .sum()
}

fn is_x_mas(matrix: &Matrix<char>, position: Coordinate) -> bool {
    if matrix.get(position).is_some_and(|&elem| elem != 'A')
        || position.row == 0
        || position.col == 0
    {
        return false;
    }

    let corners_pairs = [
        (position + (-1, -1), position + (1, 1)),
        (position + (-1, 1), position + (1, -1)),
    ];

    let chars = [('M', 'S'), ('S', 'M')];

    corners_pairs.into_iter().all(|(corner_a, corner_b)| {
        chars.into_iter().any(|(char_a, char_b)| {
            matrix.get(corner_a).is_some_and(|&elem| elem == char_a)
                && matrix.get(corner_b).is_some_and(|&elem| elem == char_b)
        })
    })
}

fn count_x_mas(matrix: &Matrix<char>) -> usize {
    matrix
        .iter_enumerate()
        .filter(|&(position, _)| is_x_mas(matrix, position))
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 4 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let matrix = read_matrix(&input)?;

    println!("Part 1: number of XMAS is {}", count_xmas(&matrix));
    println!("Part 2: number of X-MAS is {}", count_x_mas(&matrix));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_word_is_present() {
        let matrix = read_matrix("OOOS\nOOAO\nOMOO\nXOOO").unwrap();

        assert!(word_is_present(
            &matrix,
            (3, 0).into(),
            (-1, 1).into(),
            &['M', 'A', 'S']
        ));
    }

    #[test]
    fn test_not_word_is_present_at_border() {
        let matrix = read_matrix("SOOO\nAOOO\nMOOO\nOXOO").unwrap();

        assert!(!word_is_present(
            &matrix,
            (3, 1).into(),
            (-1, -1).into(),
            &['M', 'A', 'S']
        ));
    }

    #[test]
    fn test_count_xmas() {
        let matrix = read_matrix(SAMPLE).unwrap();

        assert_eq!(count_xmas(&matrix), 18);
    }

    #[test]
    fn test_is_x_mas() {
        let x_mases = [
            "M_M\n_A_\nS_S",
            "M_S\n_A_\nM_S",
            "S_S\n_A_\nM_M",
            "S_M\n_A_\nS_M",
        ];

        for x_mas in x_mases {
            let matrix = read_matrix(x_mas).unwrap();
            assert!(is_x_mas(&matrix, (1, 1).into()), "failed for:\n{}", x_mas);
        }
    }

    #[test]
    fn test_not_is_x_mas() {
        let matrix = read_matrix("M_S\n_A_\nS_M").unwrap();

        assert!(!is_x_mas(&matrix, (1, 1).into()));
    }

    #[test]
    fn test_not_is_x_mas_at_border_left() {
        let matrix = read_matrix("MS_\nA__\nMS_").unwrap();

        assert!(!is_x_mas(&matrix, (1, 0).into()));
    }

    #[test]
    fn test_not_is_x_mas_at_border_right() {
        let matrix = read_matrix("_MS\n__A\n_MS").unwrap();

        assert!(!is_x_mas(&matrix, (1, 2).into()));
    }

    #[test]
    fn test_count_x_mas() {
        let matrix = read_matrix(SAMPLE).unwrap();

        assert_eq!(count_x_mas(&matrix), 9);
    }
}
