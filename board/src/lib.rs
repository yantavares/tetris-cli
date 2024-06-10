use tetromino::Tetromino;
pub struct Board {
    pub board: Vec<Vec<char>>,
    lines: usize,
    cols: usize,
}

impl Board {
    pub fn new(lines: usize, cols: usize) -> Board {
        let board = vec![vec!['.'; cols as usize]; lines as usize];
        Board { board, lines, cols }
    }

    pub fn place_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        for &(x, y) in piece.coords() {
            let new_x = x + offset.0;
            let new_y = y + offset.1;
            if new_x < self.lines && new_y < self.cols {
                self.board[new_x][new_y] = '=';
            }
        }
    }

    pub fn clear_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        for &(x, y) in piece.coords() {
            let new_x = x + offset.0;
            let new_y = y + offset.1;
            if new_x < self.lines && new_y < self.cols {
                self.board[new_x][new_y] = '.';
            }
        }
    }

    pub fn check_collision(&self, piece: &Tetromino, offset: (usize, usize)) -> bool {
        for &(x, y) in piece.coords() {
            let new_x = x + offset.0;
            let new_y = y + offset.1;
            if new_x >= self.lines || new_y >= self.cols || self.board[new_x][new_y] == '=' {
                return true;
            }
        }
        false
    }

    pub fn solidify_piece(&mut self, piece: &Tetromino, offset: (usize, usize)) {
        self.place_piece(piece, offset);
    }

    pub fn remove_filled_lines(&mut self) {
        self.board.retain(|row| row.contains(&'.'));
        while self.board.len() < self.lines {
            self.board.insert(0, vec!['.'; self.cols]);
        }
    }
}
