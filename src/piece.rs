use std::fmt;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black")
        }
    } 
}

#[derive(Clone, Copy)]
pub enum PiecesEnum {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}

impl fmt::Display for PiecesEnum{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PiecesEnum::Pawn => write!(f, "Pawn"),
            PiecesEnum::Knight => write!(f, "Knight"),
            PiecesEnum::Bishop => write!(f, "Bishop"),
            PiecesEnum::Rook => write!(f, "Rook"),
            PiecesEnum::King => write!(f, "King"),
            PiecesEnum::Queen => write!(f, "Queen")
        }
   } 
}

pub struct Piece {
    piece: PiecesEnum,
    color: Color,
}

impl Piece {
    pub fn new(piece: PiecesEnum, color: Color) -> Self {
        Piece { piece, color }
    }
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.color, self.piece)
    }
}
