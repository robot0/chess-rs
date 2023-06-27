mod engine;

use crate::engine::game::Game;
use chess::{Square, Piece, Rank, File};


fn main() {
    let game = Game::new();

     // Print the initial board state
    println!("Initial Board State:");
    print_board(game.board());
}

// Helper function to print the board
fn print_board(board: &engine::board::Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            // Convert integers to Rank and File enums
            let rank = Rank::from_index(rank);
            let file = File::from_index(file);
            
            // Create square using Rank and File enums
            let square = Square::make_square(rank, file);
            let piece = board.piece_on(square);
            let piece_char = match piece {
                Some(p) => match p {
                    Piece::Pawn => 'P',
                    Piece::Knight => 'N',
                    Piece::Bishop => 'B',
                    Piece::Rook => 'R',
                    Piece::Queen => 'Q',
                    Piece::King => 'K',
                },
                None => '.',
            };
            print!("{} ", piece_char);
        }
        println!();
    }
}
