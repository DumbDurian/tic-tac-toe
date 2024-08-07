use super::GameState;
use std::fmt;
use tictactoe::TTTPlayer;

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, v) in self.0.board.iter().enumerate() {
            if [0, 3, 6].contains(&count) {
                write!(f, "\n")?
            }
            match v {
                TTTPlayer::None => write!(f, ". ")?,
                TTTPlayer::Human => write!(f, "x ")?,
                TTTPlayer::Computer => write!(f, "o ")?,
            }
        }
        write!(f, "\n")
    }
}
