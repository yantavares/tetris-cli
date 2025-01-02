#[derive(Debug, Clone)]
pub enum Tetromino {
    LPiece(Vec<(isize, isize)>),
    JPiece(Vec<(isize, isize)>),
    TPiece(Vec<(isize, isize)>),
    SPiece(Vec<(isize, isize)>),
    ZPiece(Vec<(isize, isize)>),
    IPiece(Vec<(isize, isize)>),
    OPiece(Vec<(isize, isize)>),
}

impl Tetromino {
    pub fn new(t: u16) -> Tetromino {
        match t {
            1 => Tetromino::LPiece(vec![(0, 0), (1, 0), (2, 0), (2, 1)]),
            2 => Tetromino::JPiece(vec![(0, 1), (1, 1), (2, 1), (2, 0)]),
            3 => Tetromino::TPiece(vec![(0, 0), (0, 1), (0, 2), (1, 1)]),
            4 => Tetromino::SPiece(vec![(0, 1), (0, 2), (1, 0), (1, 1)]),
            5 => Tetromino::ZPiece(vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
            6 => Tetromino::IPiece(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            7 => Tetromino::OPiece(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
            _ => panic!("Invalid number generated"),
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Tetromino::LPiece(_) => "L",
            Tetromino::JPiece(_) => "J",
            Tetromino::TPiece(_) => "T",
            Tetromino::SPiece(_) => "S",
            Tetromino::ZPiece(_) => "Z",
            Tetromino::IPiece(_) => "I",
            Tetromino::OPiece(_) => "O",
        }
    }

    pub fn coords(&self) -> &Vec<(isize, isize)> {
        match self {
            Tetromino::LPiece(coords)
            | Tetromino::JPiece(coords)
            | Tetromino::TPiece(coords)
            | Tetromino::SPiece(coords)
            | Tetromino::ZPiece(coords)
            | Tetromino::IPiece(coords)
            | Tetromino::OPiece(coords) => coords,
        }
    }

    pub fn rotate_right(&self) -> Tetromino {
        match self {
            Tetromino::LPiece(coords) => Tetromino::LPiece(rotate_coords(coords, true)),
            Tetromino::JPiece(coords) => Tetromino::JPiece(rotate_coords(coords, true)),
            Tetromino::TPiece(coords) => Tetromino::TPiece(rotate_coords(coords, true)),
            Tetromino::SPiece(coords) => Tetromino::SPiece(rotate_coords(coords, true)),
            Tetromino::ZPiece(coords) => Tetromino::ZPiece(rotate_coords(coords, true)),
            Tetromino::IPiece(coords) => Tetromino::IPiece(rotate_coords(coords, true)),
            Tetromino::OPiece(coords) => Tetromino::OPiece(coords.clone()),
        }
    }

    pub fn rotate_left(&self) -> Tetromino {
        match self {
            Tetromino::LPiece(coords) => Tetromino::LPiece(rotate_coords(coords, false)),
            Tetromino::JPiece(coords) => Tetromino::JPiece(rotate_coords(coords, false)),
            Tetromino::TPiece(coords) => Tetromino::TPiece(rotate_coords(coords, false)),
            Tetromino::SPiece(coords) => Tetromino::SPiece(rotate_coords(coords, false)),
            Tetromino::ZPiece(coords) => Tetromino::ZPiece(rotate_coords(coords, false)),
            Tetromino::IPiece(coords) => Tetromino::IPiece(rotate_coords(coords, false)),
            Tetromino::OPiece(coords) => Tetromino::OPiece(coords.clone()),
        }
    }
}

fn rotate_coords(coords: &Vec<(isize, isize)>, clockwise: bool) -> Vec<(isize, isize)> {
    let mut new_coords = vec![(0, 0); coords.len()];
    for (i, &(x, y)) in coords.iter().enumerate() {
        new_coords[i] = if clockwise { (y, -x) } else { (-y, x) };
    }
    new_coords
}
