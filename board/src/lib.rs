use tetromino::Tetromino;

pub struct Board {
    pub board: Vec<Vec<char>>,
    pub lines: usize,
    pub cols: usize,
    pub cleared_lines: usize,
}

impl Board {
    pub fn new(lines: usize, cols: usize) -> Board {
        let board = vec![vec!['.'; cols]; lines];
        Board {
            board,
            lines,
            cols,
            cleared_lines: 0,
        }
    }

    pub fn place_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        for &(x, y) in piece.coords() {
            let new_x = (x + offset.0 as isize) as usize;
            let new_y = (y + offset.1 as isize) as usize;
            if new_x < self.lines && new_y < self.cols {
                self.board[new_x][new_y] = '#';
            }
        }
    }

    pub fn clear_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        for &(x, y) in piece.coords() {
            let new_x = (x + offset.0 as isize) as usize;
            let new_y = (y + offset.1 as isize) as usize;
            if new_x < self.lines && new_y < self.cols {
                self.board[new_x][new_y] = '.';
            }
        }
    }

    pub fn check_collision(&self, piece: &Tetromino, offset: (usize, usize)) -> bool {
        for &(x, y) in piece.coords() {
            let new_x = (x + offset.0 as isize) as usize;
            let new_y = (y + offset.1 as isize) as usize;
            if new_x >= self.lines || new_y >= self.cols || self.board[new_x][new_y] == '#' {
                return true;
            }
        }
        false
    }

    pub fn solidify_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        self.place_piece(piece, offset);
    }

    pub fn remove_filled_lines(&mut self) {
        let mut new_board = self.board.clone();
        new_board.retain(|row| row.contains(&'.'));
        let cleared_lines = self.lines - new_board.len();
        self.cleared_lines += cleared_lines;
        while new_board.len() < self.lines {
            new_board.insert(0, vec!['.'; self.cols]);
        }
        self.board = new_board;
    }
}
