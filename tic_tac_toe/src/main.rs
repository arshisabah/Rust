use std::io;
const PLAYER_X:char = 'X';
const PLAYER_O:char = 'O';
const BOARD_SIZE:usize = 3;

//2D ARRAY - 3x3
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn init_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}
fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {} input(row,col)", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        println!("input:{}", input);

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' '{
                return (row, col);
            }
        }
        println!("Invalid input");
    }
}

fn check_winner(curr_player: char, board: &Board) -> bool {
    //row
    for row in 0..BOARD_SIZE {
        if board[row][0] == curr_player && board[row][1] == curr_player && board[row][2] == curr_player{
            return true;
        }
    }
    //col
    for col in 0..BOARD_SIZE {
        if board[0][col] == curr_player && board[1][col] == curr_player && board[2][col] == curr_player{
            return true;
        }
    }
    //diagonal
    if (board[0][0] == curr_player && board[1][1] == curr_player && board[2][2] == curr_player) ||
        (board[0][2] == curr_player && board[1][1] == curr_player) && (board[2][0] == curr_player) {
        return true;
    }
    return false;
}

fn check_draw(curr_player: char, board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    return true;
}
fn play_game() {
    let mut board = init_board();
    let mut curr_player = PLAYER_X;

    loop {
        println!("Current Board");
        print_board(&board);

        let (row,col) = get_player_move(curr_player, &board);
        board[row][col] = curr_player;

        if check_winner(curr_player,&board) {
            println!("Player {} wins!", curr_player);
            break;
        }
        if check_draw(curr_player, &board) {
            println!("The game is draw");
        }
        curr_player = if curr_player == PLAYER_X {
            PLAYER_O
        }
        else {
            PLAYER_X
        }
    }
}

fn main() {
    println!("Welcome to TicTacToe!");
    play_game();
}
