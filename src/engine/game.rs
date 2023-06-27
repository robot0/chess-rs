use crate::engine::board::Board;
use crate::engine::chess_move::ChessMove;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Game {
    
    board: Board,
    moves: Vec<ChessMove>,
    player1_name: String,
    player2_name: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            moves: Vec::new(),
            player1_name: "Player 1".to_string(),
            player2_name: "Player 2".to_string(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    // Generate all legal moves for the current position
    pub fn legal_moves(&self) -> Vec<ChessMove> {
        self.board().legal_moves()
    }

    // Make a move on the board, if it is legal
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        if self.board.is_valid_move(&chess_move) {
            self.board.make_move(chess_move.clone())?;
            self.moves.push(chess_move);
            Ok(())
        } else {
            Err("Move is not valid")
        }
    }

    pub fn set_player_names(&mut self, player1_name: &str, player2_name: &str) {
        self.player1_name = player1_name.to_string();
        self.player2_name = player2_name.to_string();
    }

}
