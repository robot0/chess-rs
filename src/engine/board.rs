use chess::{Board as ChessBoard, Color, Piece, Square};

use crate::engine::chess_move::ChessMove;
use crate::engine::movegen::MoveGenerator;

#[derive(Debug, Clone)]
pub struct Board {
    board: ChessBoard,
    movegen: MoveGenerator,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::default(),
            movegen: MoveGenerator {},
        }
    }

    // Get a reference to the inner ChessBoard instance
    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    // Generate all legal moves for the current board position
    pub fn legal_moves(&self) -> Vec<ChessMove> {
        self.movegen.generate_legal_moves(self)
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        let move_internal = chess_move.to_chess_move_internal();
        if self.movegen.is_valid_move(self, &chess_move) {
            self.board = self.board.make_move_new(move_internal);
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    // Get the piece on a given square
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        self.board.piece_on(square)
    }

    // Get the color on a given square
    pub fn color_on(&self, square: Square) -> Option<Color> {
        self.board.color_on(square)
    }

    // Validate a move on the board
    pub fn is_valid_move(&self, chess_move: &ChessMove) -> bool {
        self.movegen.is_valid_move(&self, chess_move)
    }
}
