mod engine;

use crate::engine::game::Game;
use crate::engine::chess_move::ChessMove;
use chess::{Square, Piece, Rank, File};

fn main() {
    let mut game = Game::new();

     // Print the initial board state
    println!("Initial Board State:");
    print_board(game.board());

    // Create a chess move
    // For example, move a piece from e2 to e4
    let from = Square::make_square(Rank::Second, File::E);
    let to = Square::make_square(Rank::Fourth, File::E);
    let chess_move = ChessMove::new(from, to, None);

    // Play the move
    match game.make_move(chess_move) {
        Ok(()) => println!("Move made successfully"),
        Err(err) => println!("Failed to make move: {}", err),
    }

    // Print the board state after the move
    println!("{:?}", game.board());
    print_board(game.board());

    // Now let the AI make a move
    let legal_moves = game.legal_moves();
    if !legal_moves.is_empty() {
        // In this simple example, the AI just makes the first legal move it finds
        let ai_move = legal_moves[0].clone();
        match game.make_move(ai_move) {
            Ok(()) => println!("AI made a move"),
            Err(err) => println!("AI failed to make move: {}", err),
        }

        // Print the board state after the AI move
        println!("{:?}", game.board());
    } else {
        println!("AI has no legal moves");
    }

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
