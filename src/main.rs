use board::Board;
use crossterm::{terminal, ExecutableCommand};
use rand::Rng;
use std::io::stdout;
use std::thread;
use std::time::Duration;
use tetromino::Tetromino;

fn update_board(board: &mut Board, piece: &Tetromino, offset: &mut (usize, usize)) -> bool {
    board.clear_piece(piece, *offset);
    offset.0 += 1;
    if board.check_collision(piece, *offset) {
        offset.0 -= 1;
        board.solidify_piece(piece, *offset);
        board.remove_filled_lines();
        return false;
    }
    board.place_piece(piece, *offset);
    true
}

fn display_board(board: &Board) {
    for row in board.board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let mut board = Board::new(15, 10);

    loop {
        let piece = generate_random_piece();
        let mut offset = (0, 4);

        // Place initial piece
        if board.check_collision(&piece, offset) {
            println!("Game Over!");
            break;
        }
        board.place_piece(&piece, offset);

        while update_board(&mut board, &piece, &mut offset) {
            display_board(&board);
            thread::sleep(Duration::from_millis(100));
            stdout()
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
        }
    }
}

fn generate_random_piece() -> Tetromino {
    let mut rng = rand::thread_rng();
    let n: u16 = rng.gen_range(1..8);
    Tetromino::new(n)
}
