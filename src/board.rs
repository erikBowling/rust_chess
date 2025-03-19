use crate::piece;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Row::One => write!(f, "1"),
            Row::Two => write!(f, "2"),
            Row::Three => write!(f, "3"),
            Row::Four => write!(f, "4"),
            Row::Five => write!(f, "5"),
            Row::Six => write!(f, "6"),
            Row::Seven => write!(f, "7"),
            Row::Eight => write!(f, "8"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Column::A => write!(f, "A"),
            Column::B => write!(f, "B"),
            Column::C => write!(f, "C"),
            Column::D => write!(f, "D"),
            Column::E => write!(f, "E"),
            Column::F => write!(f, "F"),
            Column::G => write!(f, "G"),
            Column::H => write!(f, "H"),
        }
    }
}

pub struct Coordinates {
    pub row: Row,
    pub column: Column,
}

impl Coordinates {
    fn new(row: Row, column: Column) -> Self {
        Coordinates { row, column }
    }
}

pub struct Square {
    pub coordinates: Coordinates,
    pub piece: Option<piece::Piece>,
}

impl Square {
    pub fn new(coordinates: Coordinates, piece: Option<piece::Piece>) -> Self {
        Square { coordinates, piece }
    }

    pub fn print(self){
        match self.piece {
            Some (p) => println!("Coodinates: {}{} Piece: {}", self.coordinates.column, self.coordinates.row, p),
            None => println!("Coodinates: {}{} Piece: {}", self.coordinates.column, self.coordinates.row, "None")
        };
    }
}

pub struct Board {
    pub squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = Vec::with_capacity(8);

        for row_id in 0..8 {
            let row_enum = match row_id {
                0 => Row::One,
                1 => Row::Two,
                2 => Row::Three,
                3 => Row::Four,
                4 => Row::Five,
                5 => Row::Six,
                6 => Row::Seven,
                7 => Row::Eight,
                _ => unreachable!(),
            };

            let mut row = Vec::with_capacity(8);
            for col_id in 0..8 {
                let col_enum = match col_id {
                    0 => Column::A,
                    1 => Column::B,
                    2 => Column::C,
                    3 => Column::D,
                    4 => Column::E,
                    5 => Column::F,
                    6 => Column::G,
                    7 => Column::H,
                    _ => unreachable!(),
                };

                let coordinates = Coordinates::new(row_enum, col_enum);
                let square = Square::new(coordinates, None);
                row.push(square)
            }

            squares.push(row);
        }

        let mut board = Board { squares };
        board.initialize_board();
        board
    }

    pub fn initialize_board(&mut self) {
        // Loop through board
        // Assign pieces correctly

        for (i, row) in self.squares.iter_mut().enumerate() {
            match i {
                0 => Self::populate_back_row(row, piece::Color::White),
                1 => Self::populate_pawns(row, piece::Color::White),
                6 => Self::populate_pawns(row, piece::Color::Black),
                7 => Self::populate_back_row(row, piece::Color::Black),
                _ => continue,
            };
        }
    }

    fn populate_back_row(row: &mut Vec<Square>, color: piece::Color) {
        for square in row {
            match square.coordinates.column {
                Column::A => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Rook, color)),
                Column::B => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Knight, color)),
                Column::C => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Bishop, color)),
                Column::D => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Queen, color)),
                Column::E => square.piece = Some(piece::Piece::new(piece::PiecesEnum::King, color)),
                Column::F => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Bishop, color)),
                Column::G => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Knight, color)),
                Column::H => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Rook, color)),
            }
        }
    }

    fn populate_pawns(row: &mut Vec<Square>, color: piece::Color) {
        for square in row.iter_mut() {
            match square.coordinates.column {
                Column::A => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::B => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::C => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::D => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::E => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::F => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::G => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
                Column::H => square.piece = Some(piece::Piece::new(piece::PiecesEnum::Pawn, color)),
            }
        }
    }
}
