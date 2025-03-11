use std::io;

fn print_board(board: [[char; 3]; 3]) {
    println!("-------------");

    for row in board {
        print!("|");
        for cell in row {
            print!(" {} ", cell);
            print!("|");
        }

        println!("\n-------------");
    }
}

fn get_valid_position(board: [[char; 3]; 3], current_player: usize, current_player_symbol: char) -> usize {
    let mut position: usize;

    loop {
        let mut user_input = String::new();
        let user_input_char: char;

        println!("Player {}. ({}) turn (select position from 1 to 9): ", current_player + 1, current_player_symbol);
        let _ = io::stdin().read_line(&mut user_input);

        if let Some(input) = user_input.chars().next() {
            user_input_char = input;
        } else {
            println!("{} is invalid string!!!", user_input);
            continue;
        }

        if let Some(digit) = user_input_char.to_digit(10) {
            position = (digit - 1) as usize;
        } else {
            println!("{} is not a digit!!!", user_input_char);
            println!("Provide single digit in range 1-9!!!");
            continue;
        }

        if board[position / 3][position % 3] == ' ' {
            break;
        } else {
            println!("Selected position is already occupied, so select another one!!!");
            continue;
        }
    }

    return position;
}

fn check_diagonals(board: [[char; 3]; 3]) -> (bool, char) {
    if (board[1][1] != ' ') &&
        ((board[0][0] == board[1][1] && board[1][1] == board[2][2]) ||
        (board[2][0] == board[1][1] && board[1][1] == board[0][2])) {
        (true, board[1][1])
    } else {
        (false, ' ')
    }
}

fn check_rows(board: [[char; 3]; 3]) -> (bool, char) {
    for idx in 0..3 {
        let mut the_same_chars_rows = true;
        let row_char = board[idx][0];

        if row_char == ' ' {
            continue;
        }

        for second_idx in 0..3 {
            if row_char != board[idx][second_idx] {
                the_same_chars_rows = false;
            }
        }

        if the_same_chars_rows {
            return (true, row_char);
        }
    }

    return (false, ' ');
}

fn check_cols(board: [[char; 3]; 3]) -> (bool, char) {
    for idx in 0..3 {
        let mut the_same_chars_cols = true;
        let col_char = board[0][idx];

        if col_char == ' ' {
            continue;
        }

        for second_idx in 0..3 {
            if col_char != board[second_idx][idx] {
                the_same_chars_cols = false;
            }
        }

        if the_same_chars_cols {
            return (true, col_char);
        }
    }

    return (false, ' ');
}

fn get_optional_winner(board: [[char; 3]; 3]) -> Option<char> {
    let (res, winner) = check_diagonals(board);

    if res {
        return Some(winner);
    }

    let (res, winner) = check_rows(board);

    if res {
        return Some(winner);
    }

    let (res, winner) = check_cols(board);

    if res {
        return Some(winner);
    }

    return None;
}

fn game() {
    const PLAYER_SYMBOL: [char; 2] = ['X', 'O'];
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];

    print_board(board);

    for turn in 0..9 {
        let current_player: usize = turn % 2;
        let current_player_symbol = PLAYER_SYMBOL[current_player];

        let user_entry = get_valid_position(board, current_player, current_player_symbol);
        let row = user_entry / 3;
        let col = user_entry % 3;

        board[row][col] = current_player_symbol;

        print_board(board);

        if let Some(winner) = get_optional_winner(board) {
            println!("Player {}. ({}) wins!!!", current_player + 1, winner);
            return;
        }

        println!();
    }

    println!("Draw!!!");
}

fn main() {
    game();
}
