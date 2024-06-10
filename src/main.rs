use board::Board;
use crossterm::{terminal, ExecutableCommand};
use rand::Rng;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;
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
        print!("\r");
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let mut board = Board::new(15, 10);

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut piece = generate_random_piece();
        let mut offset = (0, 4);

        // Place initial piece
        if board.check_collision(&piece, offset) {
            writeln!(stdout, "\rGame Over!").unwrap();
            break;
        }
        board.place_piece(&piece, offset);

        while update_board(&mut board, &piece, &mut offset) {
            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            display_board(&board);

            // Handle user input
            if let Some(Ok(b)) = stdin.next() {
                let key = b as char;
                match key {
                    'a' => {
                        // Move left
                        board.clear_piece(&piece, offset);
                        if offset.1 > 0 {
                            offset.1 -= 1;
                        }
                        if board.check_collision(&piece, offset) {
                            offset.1 += 1;
                        }
                        board.place_piece(&piece, offset);
                    }
                    'd' => {
                        // Move right
                        board.clear_piece(&piece, offset);
                        offset.1 += 1;
                        if board.check_collision(&piece, offset) {
                            offset.1 -= 1;
                        }
                        board.place_piece(&piece, offset);
                    }
                    'l' => {
                        // Rotate
                        board.clear_piece(&piece, offset);
                        let rotated_piece = piece.rotate();
                        if !board.check_collision(&rotated_piece, offset) {
                            piece = rotated_piece;
                        }
                        board.place_piece(&piece, offset);
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn generate_random_piece() -> Tetromino {
    let mut rng = rand::thread_rng();
    let n: u16 = rng.gen_range(1..8);
    Tetromino::new(n)
}
