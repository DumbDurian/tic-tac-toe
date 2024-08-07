use crate::logic::{TTTGameMove, TTTGameState, TTTPlayer};
use mcts::MCTS;

impl MCTS for TTTGameState {
    type MCTSMove = TTTGameMove;
    type MCTSState = TTTGameState;

    fn exec_move(self, gm: Self::MCTSMove) -> Self::MCTSState {
        TTTGameState::exec_move_impl(self, gm)
    }

    fn legal_moves(&self) -> Vec<Self::MCTSMove> {
        TTTGameState::legal_moves_impl(self)
    }

    fn last_move(&self) -> Self::MCTSMove {
        self.last_move.unwrap()
    }

    fn random_move(&self) -> Self::MCTSMove {
        self.get_random_move()
    }

    fn terminate(&self) -> i32 {
        let mut gs = self.clone();

        loop {
            if gs.is_terminal() {
                match gs.get_winner() {
                    TTTPlayer::Human => return 0,
                    TTTPlayer::Computer => return 2,
                    TTTPlayer::None => return 1,
                }
            }

            let rm = gs.get_random_move();
            gs = gs.exec_move(rm);
        }
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal()
    }
}
