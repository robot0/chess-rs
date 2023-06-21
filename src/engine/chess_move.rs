use chess::{ChessMove as ChessMoveInternal, Piece, Square};

#[derive(PartialEq, Clone)]
pub struct ChessMove {
    from: Square,
    to: Square,
    promotion: Option<Piece>,
}

impl ChessMove {
    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        Self {
            from,
            to,
            promotion,
        }
    }

    pub fn to_chess_move_internal(&self) -> ChessMoveInternal {
        ChessMoveInternal::new(
            self.from,
            self.to,
            self.promotion, // default value if promotion is None
        )
    }
}
