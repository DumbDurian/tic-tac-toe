use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTTPlayer {
    Computer,
    Human,
    None,
}

fn new_board() -> Vec<TTTPlayer> {
    let mut v = Vec::with_capacity(9);
    for _ in 0..9 {
        v.push(TTTPlayer::None)
    }
    v
}

fn starting_player() -> TTTPlayer {
    let mut rng = thread_rng();
    vec![TTTPlayer::Computer, TTTPlayer::Human]
        .into_iter()
        .choose(&mut rng)
        .unwrap()
}

#[derive(Clone, Debug, PartialEq)]
pub struct TTTGameState {
    pub next_player: TTTPlayer,
    pub board: Vec<TTTPlayer>,
    pub last_move: Option<TTTGameMove>,
}

impl Default for TTTGameState {
    fn default() -> Self {
        Self {
            next_player: starting_player(),
            board: new_board(),
            last_move: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTTGameMove(usize);

impl From<usize> for TTTGameMove {
    fn from(input: usize) -> Self {
        if input > 8 {
            panic!("Tried to convert a usize outside the allowed range (0-8) to a GameMove.")
        } else {
            TTTGameMove(input)
        }
    }
}

impl Into<usize> for TTTGameMove {
    fn into(self) -> usize {
        self.0
    }
}

impl TTTGameState {
    pub fn legal_moves_impl(&self) -> Vec<TTTGameMove> {
        self.board
            .iter()
            .positions(|v| *v == TTTPlayer::None)
            .map(|usi| TTTGameMove::from(usi))
            .collect()
    }

    // Internal representation of the board:
    // 0 1 2
    // 3 4 5
    // 6 7 8
    fn has_winner(&self) -> bool {
        // check columns
        for i in (0..=6).step_by(3) {
            if (self.board[i] == TTTPlayer::Human) || (self.board[i] == TTTPlayer::Computer) {
                if self.board[i] == self.board[i + 1] && self.board[i + 1] == self.board[i + 2] {
                    return true;
                }
            }
        }

        // check rows
        for i in 0..=2 {
            if (self.board[i] == TTTPlayer::Human) || (self.board[i] == TTTPlayer::Computer) {
                if self.board[i] == self.board[i + 3] && self.board[i + 3] == self.board[i + 6] {
                    return true;
                }
            }
        }

        // check diagonals
        // top left -> bottom right
        if (self.board[0] == TTTPlayer::Human) || (self.board[0] == TTTPlayer::Computer) {
            if self.board[0] == self.board[4] && self.board[4] == self.board[8] {
                return true;
            }
        }
        // top right -> bottom left
        if (self.board[2] == TTTPlayer::Human) || (self.board[2] == TTTPlayer::Computer) {
            if self.board[2] == self.board[4] && self.board[4] == self.board[6] {
                return true;
            }
        }

        false
    }

    pub fn is_draw(&self) -> bool {
        if self.legal_moves_impl().len() == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_terminal(&self) -> bool {
        if self.has_winner() {
            return true;
        } else if self.is_draw() {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_winner(&self) -> TTTPlayer {
        assert!(self.is_terminal());

        if self.is_draw() {
            return TTTPlayer::None;
        } else {
            match self.next_player {
                // Even after making a winning move the player is swapped
                // so if the Human made a winning move, the computer would
                // be the next player, but we have to return the human as
                // winner instead.
                TTTPlayer::Human => return TTTPlayer::Computer,
                TTTPlayer::Computer => return TTTPlayer::Human,
                TTTPlayer::None => panic!(),
            }
        }
    }

    pub fn exec_move_impl(mut self, gm: TTTGameMove) -> TTTGameState {
        if self.board[Into::<usize>::into(gm)] == TTTPlayer::None {
            self.board[Into::<usize>::into(gm)] = self.next_player;

            self.last_move = Some(gm);

            match self.next_player {
                TTTPlayer::Human => self.next_player = TTTPlayer::Computer,
                TTTPlayer::Computer => self.next_player = TTTPlayer::Human,
                TTTPlayer::None => panic!(),
            }

            self
        } else {
            panic!("Tried to execute a move, but the field was already taken.")
        }
    }

    pub fn get_random_move(&self) -> TTTGameMove {
        let mut rng = thread_rng();
        self.legal_moves_impl()
            .into_iter()
            .choose(&mut rng)
            .unwrap()
    }
}
