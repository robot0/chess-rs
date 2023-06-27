use std::cmp;
use crate::engine::board::Board;
use crate::engine::game::Game;

fn minimax(board: &Board, depth: i32, maximizing_player: bool) -> i32 {
    if depth == 0 {
        return evaluate_board(board);  // You'll need to implement this function
    }

    if maximizing_player {
        let mut max_eval = i32::MIN;
        let legal_moves = board.legal_moves();

        for chess_move in legal_moves {
            let mut new_board = board.clone();
            new_board.make_move(chess_move);  // You might need to modify make_move to return a new Board
            let eval = minimax(&new_board, depth - 1, false);
            max_eval = cmp::max(max_eval, eval);
        }

        return max_eval;
    } else {
        let mut min_eval = i32::MAX;
        let legal_moves = board.legal_moves();

        for chess_move in legal_moves {
            let mut new_board = board.clone();
            new_board.make_move(chess_move);  // You might need to modify make_move to return a new Board
            let eval = minimax(&new_board, depth - 1, true);
            min_eval = cmp::min(min_eval, eval);
        }

        return min_eval;
    }
}

fn evaluate_board(board: &Board) -> i32 {
    let piece_values = [
        (chess::Piece::Pawn, 1),
        (chess::Piece::Knight, 3),
        (chess::Piece::Bishop, 3),
        (chess::Piece::Rook, 5),
        (chess::Piece::Queen, 9),
        (chess::Piece::King, 0),
    ];
    let mut score = 0;

    for square in chess::ALL_SQUARES.iter() {
        if let Some(piece) = board.board().piece_on(*square) {
            for (piece_type, value) in piece_values.iter() {
                if *piece_type == piece {
                    // Add or subtract the piece value based on the piece's color
                    score += value * match board.board().color_on(*square).unwrap() {
                        chess::Color::White => 1,
                        chess::Color::Black => -1,
                    };
                }
            }

            // Add a small bonus for controlling the center
            if *square == chess::Square::E4
                || *square == chess::Square::D4
                || *square == chess::Square::E5
                || *square == chess::Square::D5
            {
                score += 1;
            }
        }
    }

    score
}


fn ai_make_move(game: &mut Game) {
    let legal_moves = game.legal_moves();
    let mut best_score = i32::MIN;
    let mut best_move = legal_moves[0].clone();  // Placeholder

    for chess_move in legal_moves {
        let mut new_game = game.clone();
        new_game.make_move(chess_move.clone());
        let score = minimax(&new_game.board(), 3, true);  // Depth of 3 as an example
        if score > best_score {
            best_score = score;
            best_move = chess_move;
        }
    }

    game.make_move(best_move);
}
