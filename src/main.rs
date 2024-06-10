use crossterm::{terminal, ExecutableCommand};
use rand::Rng;
use std::io::stdout;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Tetromino {
    LPiece,
    JPiece,
    TPiece,
    SPiece,
    ZPiece,
    IPiece,
    OPiece,
}

struct Board {
    board: Vec<Vec<char>>,
    lines: usize,
    cols: usize,
}
impl Board {
    fn new(lines: usize, cols: usize) -> Board {
        let board = vec![vec!['.'; cols as usize]; lines as usize];
        Board { board, lines, cols }
    }
}

fn update_board(board: &mut Board) -> bool {
    let mut new_board: Vec<Vec<char>> = vec![vec!['.'; board.lines]; board.cols];
    let mut has_changed = false;

    for i in 0..board.lines - 1 {
        for j in 0..board.cols - 1 {
            if board.board[i][j] == '=' {
                if board.board[i + 1][j] == '.' {
                    new_board[i + 1][j] = '=';
                    has_changed = true;
                } else {
                    continue;
                }
            }
        }
    }

    board.board = new_board;
    return has_changed;
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
        let p: Tetromino = generate_random_piece();
        println!("Random piece: {:?}", p);

        match p {
            Tetromino::LPiece => {
                board.board[0][0] = '=';
                board.board[1][0] = '=';
                board.board[2][0] = '=';
                board.board[2][1] = '=';
            }

            Tetromino::JPiece => {
                board.board[0][0] = '=';
                board.board[1][0] = '=';
                board.board[2][0] = '=';
                board.board[2][1] = '=';
            }

            Tetromino::TPiece => {
                board.board[0][0] = '=';
                board.board[0][1] = '=';
                board.board[0][2] = '=';
                board.board[1][1] = '=';
            }

            Tetromino::SPiece => {
                board.board[0][1] = '=';
                board.board[0][2] = '=';
                board.board[1][0] = '=';
                board.board[1][1] = '=';
            }

            Tetromino::ZPiece => {
                board.board[0][0] = '=';
                board.board[0][1] = '=';
                board.board[1][1] = '=';
                board.board[1][2] = '=';
            }

            Tetromino::IPiece => {
                board.board[0][0] = '=';
                board.board[1][0] = '=';
                board.board[2][0] = '=';
                board.board[3][0] = '=';
            }

            Tetromino::OPiece => {
                board.board[0][0] = '=';
                board.board[0][1] = '=';
                board.board[1][0] = '=';
                board.board[1][1] = '=';
            }
        }

        while update_board(&mut board) {
            display_board(&board);
            thread::sleep(Duration::from_secs(1));
            stdout()
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
        }
        display_board(&board);
    }
}

fn generate_random_piece() -> Tetromino {
    let mut rng = rand::thread_rng();
    let n: u16 = rng.gen_range(1..8);

    match n {
        1 => Tetromino::LPiece,
        2 => Tetromino::JPiece,
        3 => Tetromino::TPiece,
        4 => Tetromino::SPiece,
        5 => Tetromino::ZPiece,
        6 => Tetromino::IPiece,
        7 => Tetromino::OPiece,
        _ => panic!("Invalid number generated"),
    }
}
