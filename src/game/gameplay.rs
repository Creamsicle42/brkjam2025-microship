use crate::game::ActiveState;

use super::{
    microgames::{always_win, gen_new_microgame, Microgames},
    FrameInput, GameEvents, GameState,
};
use macroquad::prelude::*;

enum MicrogameState {
    TransOut(f32),
    TransIn(f32),
    InMicrogame(f32),
}

pub struct InGameData {
    microgames_completed: u8,
    lives: u8,
    current_microgame_win: bool,
    microgame_state: MicrogameState,
    current_microgame: Microgames,
}

impl Default for InGameData {
    fn default() -> Self {
        InGameData {
            microgames_completed: 0,
            lives: 3,
            current_microgame_win: false,
            microgame_state: MicrogameState::TransIn(1.0),
            current_microgame: gen_new_microgame(),
        }
    }
}

pub fn update(
    game_data: &mut GameState,
    input: FrameInput,
    delta: f32,
    events: &mut Vec<GameEvents>,
) -> Result<(), ()> {
    if let ActiveState::InGame(gs_data) = &mut game_data.active_state {
        let state_time = match &mut gs_data.microgame_state {
            MicrogameState::TransIn(t) => {
                *t -= delta;
                *t
            }
            MicrogameState::TransOut(t) => {
                *t -= delta;
                *t
            }
            MicrogameState::InMicrogame(t) => {
                *t -= delta;
                *t
            }
        };

        let microgame_won = match &mut gs_data.current_microgame {
            Microgames::AlwaysWin(d) => always_win::update(d, input, delta),
            _ => true,
        };

        if state_time <= 0.0 {
            gs_data.microgame_state = match gs_data.microgame_state {
                MicrogameState::TransIn(_) => {
                    if gs_data.microgames_completed > 9 {
                        MicrogameState::InMicrogame(3.0)
                    } else if gs_data.microgames_completed > 4 {
                        MicrogameState::InMicrogame(4.0)
                    } else {
                        MicrogameState::InMicrogame(5.0)
                    }
                }
                MicrogameState::InMicrogame(_) => {
                    if !microgame_won {
                        gs_data.lives -= 1;
                    }
                    gs_data.microgames_completed += 1;
                    MicrogameState::TransOut(0.5)
                }
                MicrogameState::TransOut(_) => {
                    gs_data.current_microgame = gen_new_microgame();
                    if gs_data.microgames_completed == 15 && gs_data.lives > 0 {
                        events.push(GameEvents::GameWon);
                    }
                    if gs_data.lives <= 0 {
                        events.push(GameEvents::GameLost);
                    }
                    MicrogameState::TransIn(0.5)
                }
            }
        }

        return Ok(());
    } else {
        return Err(());
    }
}

pub fn draw(game_data: &GameState) -> Result<(), ()> {
    if let ActiveState::InGame(gs_data) = &game_data.active_state {
        match &gs_data.current_microgame {
            Microgames::AlwaysWin(d) => always_win::draw(d),
            _ => {}
        }

        // Draw UI
        if let MicrogameState::InMicrogame(t) = gs_data.microgame_state {
            draw_text(
                format!("Time Left: {}", t).as_str(),
                16.0,
                42.0,
                32.0,
                BLACK,
            );
        } else {
            draw_text("Hold it!", 16.0, 42.0, 32.0, BLACK);
        }

        draw_text(
            format!("Lives: {}", gs_data.lives).as_str(),
            16.0,
            42.0 + 40.0,
            32.0,
            BLACK,
        );
        draw_text(
            format!("Completed: {}", gs_data.microgames_completed).as_str(),
            16.0,
            42.0 + 80.0,
            32.0,
            BLACK,
        );

        return Ok(());
    } else {
        return Err(());
    }
}
