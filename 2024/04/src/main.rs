use std::io::Read;

fn read_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn word_is_present(
    matrix: &Vec<Vec<char>>,
    from_position: (usize, usize),
    offset: (isize, isize),
    word: &[char],
) -> bool {
    let Some(char_to_compare) = word.get(0) else {
        return true;
    };

    let (from_row, from_col) = from_position;
    let (row_offset, col_offset) = offset;

    let row = (from_row as isize + row_offset) as usize;
    let col = (from_col as isize + col_offset) as usize;

    let Some(current) = matrix.get(row).and_then(|row| row.get(col)) else {
        return false;
    };

    *current == *char_to_compare && word_is_present(matrix, (row, col), offset, &word[1..])
}

fn count_xmas(matrix: &Vec<Vec<char>>) -> usize {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, &elem)| (elem == 'X').then_some((row_idx, col_idx)))
        })
        .map(|from_position| {
            (-1..=1)
                .flat_map(|row_offset| (-1..=1).map(move |col_offset| (row_offset, col_offset)))
                .filter(|&offset| word_is_present(matrix, from_position, offset, &['M', 'A', 'S']))
                .count()
        })
        .sum()
}

fn is_x_mas(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    fn is_char(matrix: &Vec<Vec<char>>, (row, col): (usize, usize), c: char) -> bool {
        matrix
            .get(row)
            .and_then(|row| row.get(col))
            .is_some_and(|&elem| elem == c)
    }

    let (row_idx, col_idx) = position;

    if row_idx == 0 || col_idx == 0 {
        return false;
    }

    let pairs = [
        ((row_idx - 1, col_idx - 1), (row_idx + 1, col_idx + 1)),
        ((row_idx - 1, col_idx + 1), (row_idx + 1, col_idx - 1)),
    ];

    let chars = [('M', 'S'), ('S', 'M')];

    pairs.into_iter().all(|(corner_a, corner_b)| {
        chars.into_iter().any(|(char_a, char_b)| {
            is_char(matrix, corner_a, char_a) && is_char(matrix, corner_b, char_b)
        })
    })
}

fn count_x_mas(matrix: &Vec<Vec<char>>) -> usize {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, col)| (*col == 'A').then_some((row_idx, col_idx)))
        })
        .filter(|&position| is_x_mas(matrix, position))
        .count()
}

fn main() {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 4 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let matrix = read_matrix(&input);

    println!("Part 1: number of XMAS is {}", count_xmas(&matrix));
    println!("Part 2: number of X-MAS is {}", count_x_mas(&matrix));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_word_is_present() {
        let matrix = read_matrix("OOOS\nOOAO\nOMOO\nXOOO");

        assert!(word_is_present(&matrix, (3, 0), (-1, 1), &['M', 'A', 'S']));
    }

    #[test]
    fn test_count_xmas() {
        let matrix = read_matrix(SAMPLE);

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
            let matrix = read_matrix(x_mas);
            assert!(is_x_mas(&matrix, (1, 1)), "failed for:\n{}", x_mas);
        }
    }

    #[test]
    fn test_not_is_x_mas() {
        let matrix = read_matrix("M_S\n_A_\nS_M");

        assert!(!is_x_mas(&matrix, (1, 1)));
    }

    #[test]
    fn test_not_is_x_mas_at_border_left() {
        let matrix = read_matrix("MS_\nA__\nMS_");

        assert!(!is_x_mas(&matrix, (1, 0)));
    }

    #[test]
    fn test_not_is_x_mas_at_border_right() {
        let matrix = read_matrix("_MS\n__A\n_MS");

        assert!(!is_x_mas(&matrix, (1, 2)));
    }

    #[test]
    fn test_count_x_mas() {
        let matrix = read_matrix(SAMPLE);

        assert_eq!(count_x_mas(&matrix), 9);
    }
}
