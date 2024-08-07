use crate::mcts::{Arena, MCTS};
use std::fmt::Debug;

impl<T: Clone + Debug + MCTS<MCTSState = T>> Arena<T> {
    fn node_expansion(&mut self, node: usize) {
        let lm = self[node].game_state.legal_moves().into_iter();

        for m in lm {
            self.generate_child_for_move(node, m);
            // let child_id = self.next_id();
        }
    }

    fn generate_child_for_move(&mut self, node: usize, gm: <T as MCTS>::MCTSMove) {
        let mut gs = self[node].game_state.clone();
        gs = gs.exec_move(gm);
        self.child_from_node_with_game_state(node, gs);
    }

    fn determine_next_node(&self) -> usize {
        let mut current = 0;

        while !self[current].is_leaf() {
            current = *self[current]
                .children
                .iter()
                .max_by_key(|child| self.ucb1_of(**child))
                .unwrap();
        }

        return current;
    }

    fn process_node(&mut self, node: usize) {
        if (self[node].visits == 0) | (self[node].game_state.is_terminal()) {
            self.rollout(node);
        } else {
            self.node_expansion(node);
        }
    }

    fn rollout(&mut self, node: usize) {
        let game_score = self[node].game_state.terminate();
        self[node].score += game_score;
        self[node].visits += 1;
        self.propagate_from(node, game_score);
    }

    fn propagate_from(&mut self, node: usize, game_score: i32) {
        let mut node = node;

        loop {
            match self[node].parent {
                Some(parent) => {
                    self[parent].score += game_score;
                    self[parent].visits += 1;
                    node = self[parent].id;
                }
                None => break,
            }
        }
    }

    pub(crate) fn iterate_once(&mut self) {
        let node = self.determine_next_node();
        self.process_node(node);
    }
}
