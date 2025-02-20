use game::{
    draw_game_state, gather_frame_input, get_texture_images, init_game_state, update_game_state,
    GameState,
};
use macroquad::{prelude::*, rand, time, window::Conf};
use std::{sync::mpsc, thread};

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

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        let _ = tx.send(get_texture_images());
    });

    let mut state: GameState;
    let mut load_time = 0.0;

    'load: loop {
        if let Ok(t) = rx.try_recv() {
            state = init_game_state(t);
            break 'load;
        }
        load_time += get_frame_time();
        clear_background(BLACK);
        draw_text(
            "Loading...",
            16.0,
            64.0 + 16.0 * load_time.sin(),
            32.0,
            WHITE,
        );
        next_frame().await;
    }

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
