#[derive(Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
enum PieceColor {
    Black,
    White,
}

struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}

#[derive(Debug)]
pub enum Coordinates {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

pub struct Square {
    coordinates: Coordinates,
}

pub struct Board {
    squares: Vec<Square>,
    next_move: Player,
}

#[derive(Debug)]
struct Player(PieceColor);

struct Pos;

impl Default for Board {
    fn default() -> Board {
        use self::Square;
        use self::Coordinates::{A1, A2};
        use self::Piece;
        use self::Player;
        use self::PieceColor;


        Board {
            squares: vec![Square { coordinates: A1 }],
            next_move: Player(PieceColor::White),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_has_first_move() {
        let board = Board { ..Default::default() };
        assert_eq!(board.next_move.0, PieceColor::White);
    }
}

