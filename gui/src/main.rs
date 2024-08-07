use mcts::sim;
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::cell::RefCell;
use std::{rc::Rc, thread};
use tictactoe::{TTTGameMove, TTTGameState, TTTPlayer};
slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    // Init the UI
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    let empty_fields = empty_fields(&ui);

    let field_model = new_model(&empty_fields, &ui);

    // Init the game logic
    let game = Rc::new(RefCell::new(new_game(&field_model.clone())));

    let game_weak = game.clone();
    let field_model_weak = field_model.clone();

    ui.global::<Logic>().on_restart(move || {
        let mut game = game_weak.borrow_mut();

        field_model_weak
            .as_any()
            .downcast_ref::<VecModel<DataField>>()
            .unwrap()
            .set_vec(empty_fields.clone());
        *game = new_game(&field_model_weak.clone());
    });

    let game_weak = game.clone();
    let field_model_weak = field_model.clone();

    ui.global::<Logic>().on_user_input(move |id| {
        let mut game = game_weak.borrow_mut();

        if !game.is_terminal() {
            let gm = TTTGameMove::from(id as usize);

            if !game.legal_moves_impl().contains(&gm.into()) {
                return;
            }

            *game = game.clone().exec_move_impl(gm);

            field_model_weak.set_row_data(
                id as usize,
                DataField {
                    player: Player::X,
                    disable_ta: true,
                },
            );

            println!("Field number {}, was clicked", id);
        }

        if !game.is_terminal() {
            let computer_move = sim(&*game, 10_000);
            *game = game.clone().exec_move_impl(computer_move);

            field_model_weak.set_row_data(
                computer_move.into(),
                DataField {
                    player: Player::O,
                    disable_ta: true,
                },
            );
        }

        if game.is_terminal() {
            for i in 0..field_model_weak.row_count() {
                let mut item = field_model_weak.row_data(i).unwrap();
                item.disable_ta = true;
                field_model_weak.set_row_data(i, item);
                field_model_weak.model_tracker();
            }

            if game.is_draw() {
                if let Some(handle) = ui_weak.upgrade() {
                    handle.invoke_show_result("A draw it is. For now...".into());
                    handle.window().request_redraw();
                }
            } else {
                match game.get_winner() {
                    TTTPlayer::Human => {
                        if let Some(handle) = ui_weak.upgrade() {
                            handle.invoke_show_result(
                                "You have defeated me... this cannot be!".into(),
                            );
                        }
                    }
                    TTTPlayer::Computer => {
                        if let Some(handle) = ui_weak.upgrade() {
                            handle
                                .invoke_show_result("I won human, you can never defeat me!".into());
                        }
                    }
                    TTTPlayer::None => panic!(),
                }
            }
        }
    });

    _ = ui.run();
}

fn new_game(field_model: &ModelRc<DataField>) -> TTTGameState {
    // Rc<RefCell<TTTGameState>> {
    let mut game = TTTGameState::default();
    match game.next_player {
        TTTPlayer::Computer => {
            let cm = sim(&game, 10_000);
            game = game.exec_move_impl(cm);
            field_model.set_row_data(
                cm.into(),
                DataField {
                    player: Player::O,
                    disable_ta: true,
                },
            );
        }
        _ => (),
    }

    game
}

fn new_model(fields: &Vec<DataField>, ui: &AppWindow) -> ModelRc<DataField> {
    let model = std::rc::Rc::new(VecModel::from(fields.clone()));
    let field_model: ModelRc<_> = model.clone().into();
    ui.set_data_model(field_model.clone().into());
    field_model
}

fn empty_fields(ui: &AppWindow) -> Vec<DataField> {
    let mut fields: Vec<DataField> = ui.get_data_model().iter().collect();
    for _ in 0..9 {
        fields.push(DataField {
            player: Player::None,
            disable_ta: false,
        });
    }
    fields
}
