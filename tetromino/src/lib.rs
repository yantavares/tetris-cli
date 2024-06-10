#[derive(Debug, Clone)]
pub enum Tetromino {
    LPiece(Vec<(usize, usize)>),
    JPiece(Vec<(usize, usize)>),
    TPiece(Vec<(usize, usize)>),
    SPiece(Vec<(usize, usize)>),
    ZPiece(Vec<(usize, usize)>),
    IPiece(Vec<(usize, usize)>),
    OPiece(Vec<(usize, usize)>),
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

    pub fn contains(&self, position: (usize, usize)) -> bool {
        match self {
            Tetromino::LPiece(coords)
            | Tetromino::JPiece(coords)
            | Tetromino::TPiece(coords)
            | Tetromino::SPiece(coords)
            | Tetromino::ZPiece(coords)
            | Tetromino::IPiece(coords)
            | Tetromino::OPiece(coords) => coords.contains(&position),
        }
    }

    pub fn coords(&self) -> &Vec<(usize, usize)> {
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

    pub fn rotate(&self) -> Tetromino {
        match self {
            Tetromino::LPiece(coords) => Tetromino::LPiece(rotate_coords(coords)),
            Tetromino::JPiece(coords) => Tetromino::JPiece(rotate_coords(coords)),
            Tetromino::TPiece(coords) => Tetromino::TPiece(rotate_coords(coords)),
            Tetromino::SPiece(coords) => Tetromino::SPiece(rotate_coords(coords)),
            Tetromino::ZPiece(coords) => Tetromino::ZPiece(rotate_coords(coords)),
            Tetromino::IPiece(coords) => Tetromino::IPiece(rotate_coords(coords)),
            Tetromino::OPiece(coords) => Tetromino::OPiece(coords.clone()), // O-piece does not change on rotation
        }
    }
}

fn rotate_coords(coords: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_coords = vec![(0, 0); coords.len()];
    for (i, &(x, y)) in coords.iter().enumerate() {
        new_coords[i] = (y, 3 - x); // Rotate 90 degrees clockwise
    }
    new_coords
}
