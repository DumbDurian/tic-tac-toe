use tictactoe::{TTTPlayer, TTTGameState};
use mcts::sim;
use crate::parse::rec_get_human_move;
mod parse;
mod display;

struct GameState(TTTGameState);

impl From<&TTTGameState> for GameState {
    fn from(value: &TTTGameState) -> Self {
        GameState(value.clone())
    }
}

fn main() {

    let mut game = TTTGameState::default();
    println!("{}",GameState::from(&game));

    loop {

        if game.is_terminal() {
            if game.is_draw() {
                println!("A draw it is. For now...");
                std::process::exit(0);
            } else {
                match game.get_winner() {
                TTTPlayer::Human => {
                    println!("\n You won human, this cannot be!");
                    std::process::exit(0)
                },
                TTTPlayer::Computer => {
                    println!("\n I won human, you will never defeat me!");
                    std::process::exit(0)
                },
                TTTPlayer::None => panic!(),
                }
            }
        }

        match game.next_player {
            TTTPlayer::Human => {
                let hm = rec_get_human_move(&game);
                game = game.exec_move_impl(hm).into();
            },
            TTTPlayer::Computer => {
                // let cm = game.get_random_move();
                let cm = sim(&game, 10_000);
                game = game.exec_move_impl(cm);
            }
            TTTPlayer::None => panic!(),
        }

        println!("{}", GameState::from(&game));

    }

}




