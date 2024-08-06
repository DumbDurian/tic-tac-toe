use tictactoe::{TTTGameMove, TTTGameState};

fn parse_input() -> Option<usize> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");

    match input.trim().parse::<usize>() {
        Ok(idx) => match idx {
            1..=9 => Some(idx-1),
            _ => {
                println!("Please enter a single digit between 1 and 9.");
                None
            },
        }
        Err(_) => {
            println!("Please enter a single digit between 1 and 9.");
            None
        }
    }
}


pub fn rec_get_input() -> Option<usize> {
    match parse_input() {
        Some(x) => Some(x),
        None => rec_get_input(),
    }
}

pub fn rec_get_human_move(gs: &TTTGameState) -> TTTGameMove {
    match rec_get_input() {
        Some(x) => {
            if gs.legal_moves_impl().contains(&x.into()) {
                TTTGameMove::from(x)
            } else {
                println!("Field {:?} already taken!", &x+1);
                rec_get_human_move(gs)
            }
        },
        None => rec_get_human_move(gs),
    }
}


// impl GameState {

//     pub fn rec_get_human_move(&self) -> TTTGameMove {
//         match rec_get_input() {
//             Some(x) => {
//                 if self.legal_moves_impl().contains(&x.into()) {
//                     GameMove::from(x)
//                 } else {
//                     println!("Field {:?} already taken!", &x+1);
//                     self.rec_get_human_move()
//                 }
//             },
//             None => self.rec_get_human_move(),
//         }
//     }

// }

