mod engine;

use crate::engine::game::Game;

fn main() {
    let game = Game::new();
}

// fn main() {
//     let mut board = Board::new();
//     make_moves(&mut board);
// display_board(&board);
// }

// #[launch]
// fn rocket() -> _ {
//     let mut board = Board::new();
//     make_moves(&mut board);
//     // display_board(&board);
//     rocket::build().mount("/", routes![index])
// }
