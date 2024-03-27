use std::io;

fn main() {
    let mut board = [' '; 9];
    let mut player = 'X';

    let win_conditions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // rows
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // cols
        [0, 4, 8],
        [2, 4, 6], // diags
    ];

    loop {
        // Print the board
        println! {"{} | {} | {}",board[0],board[1],board[2]}
        println! {"--|---|--"}
        println! {"{} | {} | {}",board[3],board[4],board[5]}
        println! {"--|---|--"}
        println! {"{} | {} | {}",board[6],board[7],board[8]}

        // Input
        loop {
            // Get player input
            let mut input = String::new();
            println! {"Enter move 0->8 : "};
            let _ = io::stdin().read_line(&mut input);

            // Check if the move is valid and make
            let move_index: usize = input.trim().parse().unwrap_or(9);
            if move_index < 9 && board[move_index] == ' ' {
                board[move_index] = player;
                break;
            } else {
                println! {"Not a move."}
            }
        }

        // Tie
        if board.iter().all(|&cell| cell != ' ') {
            println!("TIE!");
            return;
        }

        // There is a winner
        for win in &win_conditions {
            if board[win[0]] == player && board[win[1]] == player && board[win[2]] == player {
                println!("{} WIN!", player);
                return;
            }
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' };
    }
}
