use crate::mcts::{Arena,MCTS,Node};

#[derive(Clone,Debug)]
pub struct GameStateTest {}

impl Default for GameStateTest {
    fn default() -> Self {
        GameStateTest {}
    }
}

    struct MCTSMove;
    struct MCTSState;

impl MCTS for GameStateTest {

    type MCTSMove = usize;
    type MCTSState = GameStateTest;
    
    fn legal_moves(&self) -> Vec<usize> {
        vec![0,1]
    }

    fn exec_move(self, gm: usize) -> GameStateTest {
        GameStateTest::default()
    }

    fn random_move(&self) -> usize {
        0
    }

    fn terminate(&self) -> i32 {
        2
    }

    fn last_move(&self) -> usize {
        0
    }

    fn is_terminal(&self) -> bool {
        false
    }

}
