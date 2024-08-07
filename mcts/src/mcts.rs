use crate::ucb1::average_score;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

pub trait MCTS {
    type MCTSMove: Clone;
    type MCTSState: Clone;

    fn exec_move(self, gm: Self::MCTSMove) -> Self::MCTSState;
    fn legal_moves(&self) -> Vec<Self::MCTSMove>;
    fn last_move(&self) -> Self::MCTSMove;
    fn random_move(&self) -> Self::MCTSMove;
    fn terminate(&self) -> i32;
    fn is_terminal(&self) -> bool;
    // terminate: The game should be played from it's current state with random
    // moves until it reaches a terminal state. The returned value should be
    // the value of this result. In the simplest case for example:
    // -1 for a loss, 0 for a draw, +1 for a win.
}

#[derive(Debug)]
pub(crate) struct Node<T: Debug + MCTS> {
    pub(crate) id: usize,
    pub(crate) parent: Option<usize>,
    pub(crate) game_state: T,
    pub(crate) visits: u32,
    pub(crate) score: i32,
    pub(crate) children: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct Arena<T: Debug + MCTS> {
    nodes: Vec<Node<T>>,
}

pub fn sim<T>(gs: &T, n: i32) -> T::MCTSMove
where
    T: Clone + Debug + MCTS<MCTSState = T>,
{
    let mut arena = Arena::new_arena_with_game_state(gs);

    for _ in 0..n {
        arena.iterate_once();
    }

    let proposed_node = *arena[0]
        .children
        .iter()
        .max_by_key(|child| average_score(arena[**child].score, arena[**child].visits))
        .unwrap();

    arena[proposed_node].game_state.last_move()
}

impl<T: Clone + Debug + MCTS> Node<T> {
    pub(crate) fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn new_root_with_game_state(gs: &T) -> Node<T> {
        Self {
            id: 0,
            parent: None,
            game_state: gs.clone(),
            visits: 0,
            score: 0,
            children: Vec::new(),
        }
    }
}

impl<T: Clone + Debug + MCTS> Arena<T> {
    fn new_child_with_game_state(&self, parent: usize, child_id: usize, gs: T) -> Node<T> {
        Node {
            id: child_id,
            parent: Some(parent),
            game_state: gs,
            visits: 0,
            score: 0,
            children: Vec::new(),
        }
    }

    pub(crate) fn new_arena_with_game_state(gs: &T) -> Self {
        Arena {
            nodes: vec![Node::new_root_with_game_state(gs)],
        }
    }

    // TODO: Find a better (multithreadable) solution for getting the next id.
    pub(crate) fn next_id(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn child_from_node_with_game_state(&mut self, parent: usize, gs: T) {
        let child_id = self.next_id();
        self.nodes[parent].children.push(child_id);
        self.nodes
            .push(self.new_child_with_game_state(parent, child_id, gs));
    }
}

impl<T: Debug + MCTS> Index<usize> for Arena<T> {
    type Output = Node<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T: Debug + MCTS> IndexMut<usize> for Arena<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

use crate::minimal::GameStateTest;

pub(crate) fn setup_test_arena() -> Arena<GameStateTest> {
    let mut arena = Arena::new_arena_with_game_state(&GameStateTest::default());
    arena.child_from_node_with_game_state(0, GameStateTest {});
    arena.child_from_node_with_game_state(0, GameStateTest {});
    arena.child_from_node_with_game_state(0, GameStateTest {});
    arena[0].visits = 3;
    arena[1].visits = 2;
    arena[2].visits = 1;
    arena[3].visits = 0;

    arena[0].score = 6;
    arena[1].score = 3;
    arena[2].score = 2;
    arena[3].score = 0;

    arena
}
