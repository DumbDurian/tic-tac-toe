use crate::mcts::{Arena, MCTS};
use ordered_float::OrderedFloat;
use std::fmt::Debug;

pub(crate) fn average_score(score: i32, visits: u32) -> OrderedFloat<f32> {
    OrderedFloat(score as f32 / visits as f32)
}

impl<T: Debug + MCTS> Arena<T> {
    pub(crate) fn average_score_of(&self, node: usize) -> f32 {
        self[node].score as f32 / self[node].visits as f32
    }

    pub(crate) fn ucb1_of(&self, node: usize) -> OrderedFloat<f32> {
        OrderedFloat(self.exploitation_term(node) + self.exploration_term(node))
    }

    fn exploitation_term(&self, node: usize) -> f32 {
        if self[node].score == 0 {
            return f32::MAX;
        } else {
            self[node].score as f32 / self[node].visits as f32
        }
    }

    fn exploration_nominator(&self, node: usize) -> f32 {
        match self[node].parent {
            Some(parent) => match self[parent].visits {
                0 => panic!(
                    "Parent had 0 visits,
                but visits should always be greater than 0,
                because parent had to be rolled out at least once."
                ),
                _ => ((self[parent].visits) as f32).ln(),
            },
            None => panic!(
                "Tried to calculate UCB1 of root,
            but UCB1 of root never has to be calculated."
            ),
        }
    }

    fn exploration_denominator(&self, node: usize) -> f32 {
        self[node].visits as f32
    }

    fn exploration_term(&self, node: usize) -> f32 {
        2.0 * (self.exploration_nominator(node) / self.exploration_denominator(node)).sqrt()
    }
}

#[cfg(test)]
mod tests {

    use crate::mcts::setup_test_arena;
    use crate::minimal::GameStateTest;

    #[test]
    fn exploitation_term_of_fresh_node() {
        let arena = setup_test_arena();
        let exploitation = arena.exploitation_term(3);
        assert_eq!(exploitation, f32::MAX);
    }

    #[test]
    fn exploitation_term_of_populated_node() {
        let arena = setup_test_arena();
        let exploitation = arena.exploitation_term(1);
        assert_eq!(exploitation, 1.5);
    }

    #[test]
    fn exploration_nominator() {
        let arena = setup_test_arena();
        let nominator = arena.exploration_nominator(2);
        assert_eq!(nominator, 1.0986123);
    }

    #[test]
    #[should_panic]
    fn exploration_nominator_with_zero_parent_visits() {
        let mut arena = setup_test_arena();
        arena.child_from_node_with_game_state(3, GameStateTest {});
        arena.exploration_nominator(4);
    }

    #[test]
    fn exploration_denominator_of_fresh_node() {
        let arena = setup_test_arena();
        let denominator = arena.exploration_denominator(3);
        assert_eq!(denominator, 0.0);
    }

    #[test]
    fn exploration_denominator_of_populated_node() {
        let arena = setup_test_arena();
        let denominator = arena.exploration_denominator(2);
        assert_eq!(denominator, 1.0);
    }

    #[test]
    fn exploration_term_of_fresh_node() {
        let arena = setup_test_arena();
        let exploration_term = arena.exploration_term(3);
        assert_eq!(exploration_term, f32::INFINITY)
    }

    #[test]
    fn exploration_term_of_populated_node() {
        let arena = setup_test_arena();
        let exploration_term = arena.exploration_term(2);
        assert_eq!(exploration_term, 2.0962942)
    }

    #[test]
    fn ucb1_of_fresh_node() {
        let arena = setup_test_arena();
        let ucb1 = arena.ucb1_of(3);
        assert_eq!(ucb1, f32::INFINITY)
    }

    #[test]
    fn ucb1_of_populated_node() {
        let arena = setup_test_arena();
        let ucb1 = arena.ucb1_of(2);
        assert_eq!(ucb1, 4.0962944);
        let ucb1 = arena.ucb1_of(1);
        assert_eq!(ucb1, 2.9823039)
    }
}
