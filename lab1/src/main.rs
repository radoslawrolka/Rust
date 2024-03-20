use std::io;

fn main() {
    let mut who = 1;
    let mut turn = 0;
    let mut board = [['.'; 3]; 3];

    println!("Welcome to the Tic Tac Toe game!\nBoard numbers are as follows:\n 0 1 2\n 3 4 5\n 6 7 8\n");
    print_board(&board);

    while turn < 9 {
        println!("Type your command player {}:", who);
        get_move(&mut board, who);
        print_board(&board);
        if check_win(&board) {
            break;
        }
        who = if who == 2 {1} else {2};
        turn += 1;
    }
    if turn == 9 {
        println!("DRAW!");
    } else {
        println!("Player {} wins!", who);
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for i in 0..3 {
        println!("{:?}", board[i]);
    }
}

fn get_move(board: &mut [[char; 3]; 3], who: i32) {
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    let cell = user_input.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
    board[cell/3][cell%3] = if who == 1 {'X'} else {'O'};
}

fn check_win(board: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != '.' && board[i][0] == board[i][1] && board[i][0] == board[i][2] {
            return true;
        }
        if board[0][i] != '.' && board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            return true;
        }
    }
    if board[0][0] != '.' && board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return true;
    }
    if board[0][2] != '.' && board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return true;
    }
    false
}