use chess::MoveGen;

use crate::engine::board::Board;
use crate::engine::chess_move::ChessMove;

pub struct MoveGenerator {}

impl MoveGenerator {
    // Generate all legal moves for a given board
    pub fn generate_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        let mut movegen = MoveGen::new_legal(board.board());
        let mut moves = vec![];

        while let Some(next_move) = movegen.next() {
            moves.push(ChessMove::new(
                next_move.get_source(),
                next_move.get_dest(),
                next_move.get_promotion(),
            ));
        }
        moves
    }

    // Validate a move on the given board
    pub fn is_valid_move(&self, board: &Board, chess_move: &ChessMove) -> bool {
        let mut movegen = MoveGen::new_legal(board.board());

        while let Some(next_move) = movegen.next() {
            let next_move_custom = ChessMove::new(
                next_move.get_source(),
                next_move.get_dest(),
                next_move.get_promotion(),
            );
            if next_move_custom == *chess_move {
                return true;
            }
        }

        false
    }
}
