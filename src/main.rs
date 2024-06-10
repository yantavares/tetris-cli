use board::Board;
use crossterm::{terminal, ExecutableCommand};
use rand::Rng;
use std::io::{self, Read, Write};
use std::thread;
use std::time::{Duration, Instant};
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

fn display_board<W: Write>(board: &Board, stdout: &mut W, piece: &Tetromino) {
    writeln!(stdout, "\rNext piece {}", piece.get_type()).unwrap();
    for row in board.board.iter() {
        write!(stdout, "\r").unwrap();
        for cell in row.iter() {
            write!(stdout, "{} ", cell).unwrap();
        }
        writeln!(stdout).unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    let mut board = Board::new(15, 10);

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut fall_timer = Instant::now();
    let fall_interval = Duration::from_millis(250);

    let mut piece = generate_random_piece();

    loop {
        let next_piece = generate_random_piece();

        let mut offset = (0, 4);

        // Place initial piece
        if board.check_collision(&piece, offset) {
            writeln!(stdout, "\rGame Over!").unwrap();
            println!("\rCleared Lines: {}", board.cleared_lines);
            break;
        }
        board.place_piece(&piece, offset);

        let mut running = true;
        while running {
            if fall_timer.elapsed() >= fall_interval {
                fall_timer = Instant::now();
                running = update_board(&mut board, &piece, &mut offset);
                stdout
                    .execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap();
                display_board(&board, &mut stdout, &next_piece);
            }

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
                        let rotated_piece_right = piece.rotate_right();
                        if !board.check_collision(&rotated_piece_right, offset) {
                            piece = rotated_piece_right;
                        } else {
                            let rotated_piece_left = piece.rotate_left();
                            if !board.check_collision(&rotated_piece_left, offset) {
                                piece = rotated_piece_left;
                            }
                        }
                        board.place_piece(&piece, offset);
                    }

                    'q' => {
                        // Quit the game
                        return;
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(50)); // Polling interval for input handling
        }
        piece = next_piece;
    }
}

fn generate_random_piece() -> Tetromino {
    let mut rng = rand::thread_rng();
    let n: u16 = rng.gen_range(1..8);
    Tetromino::new(n)
}
