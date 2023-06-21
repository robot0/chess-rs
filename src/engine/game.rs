use crate::engine::board::Board;
use crate::engine::chess_move::ChessMove;
use crate::engine::movegen::MoveGenerator;

pub struct Game {
    board: Board,
    moves: Vec<ChessMove>,
    movegen: MoveGenerator,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            moves: Vec::new(),
            movegen: MoveGenerator {},
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    // Generate all legal moves for the current position
    pub fn legal_moves(&self) -> Vec<ChessMove> {
        self.movegen.generate_legal_moves(&self.board)
    }

    // Make a move on the board, if it is legal
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        if self.movegen.is_valid_move(&self.board, &chess_move) {
            self.board.make_move(chess_move.clone()); // Clones the chess_move before passing it
                                                      // use of moved value: `chess_move` - value used here after move
            self.moves.push(chess_move);
            Ok(())
        } else {
            Err("Move is not valid")
        }
    }
}
