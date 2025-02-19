use game::{draw_game_state, gather_frame_input, init_game_state, update_game_state};
use macroquad::{prelude::*, rand, time, window::Conf};

mod game;

fn game_config() -> Conf {
    Conf {
        window_title: "Microship".to_string(),
        window_resizable: false,
        window_width: 960,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(game_config)]
async fn main() {
    rand::srand(time::get_time() as u64);

    clear_background(BLACK);
    draw_text("Loading...", 16.0, 48.0, 32.0, WHITE);
    next_frame().await;

    let mut state = init_game_state().await;

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
