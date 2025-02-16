use game::{draw_game_state, gather_frame_input, init_game_state, update_game_state};
use macroquad::prelude::*;

mod game;

#[macroquad::main("Microship")]
async fn main() {
    let mut state = init_game_state();

    'game: loop {
        let res = update_game_state(&mut state, gather_frame_input(), get_frame_time());
        if res.is_err() {
            break 'game;
        }
        let res = draw_game_state(&state);
        if res.is_err() {
            break 'game;
        }

        next_frame().await
    }
}
