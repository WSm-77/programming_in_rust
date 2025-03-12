use std::collections::HashSet;

fn check_digits_range(board: [[u8; 9]; 9]) -> bool {
    for row in board {
        for cell in row {
            if cell > 9 {
                return false;
            }
        }
    }

    return true;
}

fn check_rows(board: [[u8; 9]; 9]) -> bool {
    for row in board {
        let mut digits_in_row: HashSet<u8> = HashSet::new();

        for cell in row {
            if cell == 0 {
                continue;
            }

            if digits_in_row.contains(&cell) {
                return false;
            }

            digits_in_row.insert(cell);
        }
    }

    return true;
}

fn check_cols(board: [[u8; 9]; 9]) -> bool {
    let n = board.len();

    for col_idx in 0..n {
        let mut digits_in_col: HashSet<u8> = HashSet::new();

        for row_idx in 0..n {
            let cell = board[row_idx][col_idx];

            if cell == 0 {
                continue;
            }

            if digits_in_col.contains(&cell) {
                return false;
            }

            digits_in_col.insert(cell);
        }
    }

    return true;
}

fn check_small_squares(board: [[u8; 9]; 9]) -> bool {
    let n = board.len();

    for small_sqr_idx in 0..n {
        let row_offset = (small_sqr_idx  / 3) * 3;
        let col_offset = (small_sqr_idx % 3) * 3;
        let mut digits_in_small_sqr: HashSet<u8> = HashSet::new();

        for cell_idx in 0..n {
            let row_idx = row_offset + (cell_idx / 3);
            let col_idx = col_offset + (cell_idx % 3);

            let cell = board[row_idx][col_idx];

            if cell == 0 {
                continue;
            }

            if digits_in_small_sqr.contains(&cell) {
                return false;
            }

            digits_in_small_sqr.insert(cell);
        }
    }

    return true;
}

fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {
    return check_digits_range(board) &&
            check_rows(board) &&
            check_cols(board) &&
            check_small_squares(board);
}

fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];

    println!("{}", check_sudoku_board(board));

    println!("Run cargo test to test check_sudoku_board");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_valid_sudoku_board() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];

        assert!(check_sudoku_board(board));
    }

    #[test]
    fn test_sudoku_board_with_invalid_row() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 8, 0, 0, 6, 0],    // invalid row
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];

        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_sudoku_board_with_invalid_col() {
        // invalid 8-th col (doubled 7)
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 7, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];

        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_sudoku_board_with_invalid_small_square() {
        // 1-th small square (rows 0-2, cols 0-2) doubled 6
        let board = [
            [5, 3, 6, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];

        assert!(!check_sudoku_board(board));

        // 8-th small square (rows 6-8, cols 6-8) doubled 1
        let board = [
            [5, 3, 0,   0, 7, 0,     0, 0, 0],
            [6, 0, 0,   1, 9, 5,     0, 0, 0],
            [0, 9, 8,   0, 0, 0,     0, 6, 0],

            [8, 0, 0,   0, 6, 0,     0, 0, 3],
            [4, 0, 0,   8, 0, 3,     0, 0, 1],
            [7, 0, 0,   0, 2, 0,     0, 0, 6],
                      // doubled 1
            [0, 6, 0,   0, 0, 1,     2, 8, 0],
            [0, 0, 0,   4, 1, 9,     0, 0, 5],
            [0, 0, 0,   0, 8, 0,     0, 7, 9]
        ];

        assert!(!check_sudoku_board(board));
    }
}
