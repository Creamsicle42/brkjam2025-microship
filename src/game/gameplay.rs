use crate::game::ActiveState;

use super::{
    microgames::{
        always_win, asteroids, combo, course, crank, gen_microgame, gen_microgame_queue,
        gen_new_microgame, imposter, pipes, swap, sweep, Microgames,
    },
    particles_draw, particles_update, FrameInput, GameEvents, GameState, MousePressState,
};
use macroquad::prelude::*;

#[derive(Debug, PartialEq)]
enum MicrogameState {
    TransOut(f32),
    TransIn(f32),
    InMicrogame(f32),
}

enum EndPlateState {
    None,
    WonMG(u8),
    LostMG(u8),
}

impl MicrogameState {
    fn is_in_microgame(&self) -> bool {
        match self {
            MicrogameState::InMicrogame(_) => true,
            _ => false,
        }
    }
}

pub struct InGameData {
    microgames_completed: u8,
    lives: u8,
    current_microgame_win: bool,
    microgame_state: MicrogameState,
    current_microgame: Microgames,
    game_queue: Vec<u8>,
    end_plate: EndPlateState,
}

impl Default for InGameData {
    fn default() -> Self {
        let mut game_queue = gen_microgame_queue();
        InGameData {
            end_plate: EndPlateState::None,
            microgames_completed: 0,
            lives: 3,
            current_microgame_win: false,
            microgame_state: MicrogameState::TransIn(0.5),
            current_microgame: gen_microgame(game_queue.pop().unwrap()),
            game_queue,
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

        let filtered_input: FrameInput = if gs_data.microgame_state.is_in_microgame() {
            input
        } else {
            FrameInput {
                mouse_position: Vec2::ZERO,
                mouse_state: MousePressState::NotPressed,
            }
        };

        let microgame_won = match &mut gs_data.current_microgame {
            Microgames::AlwaysWin(d) => always_win::update(d, filtered_input, delta),
            Microgames::Pipes(d) => pipes::update(d, filtered_input, delta),
            Microgames::Combo(d) => combo::update(d, filtered_input, delta),
            Microgames::Course(d) => course::update(d, filtered_input, delta),
            Microgames::Sweep(d) => sweep::update(d, filtered_input, delta),
            Microgames::Asteroids(d) => asteroids::update(d, filtered_input, delta),
            Microgames::Imposter(d) => imposter::update(d, filtered_input, delta),
            Microgames::Swap(d) => swap::update(d, filtered_input, delta),
            Microgames::Crank(d) => crank::update(d, filtered_input, delta),
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
                        gs_data.end_plate = EndPlateState::LostMG(1);
                    } else {
                        gs_data.end_plate = EndPlateState::WonMG(1);
                    }
                    gs_data.microgames_completed += 1;
                    MicrogameState::TransOut(0.75)
                }
                MicrogameState::TransOut(_) => {
                    gs_data.current_microgame = gen_microgame(gs_data.game_queue.pop().unwrap());
                    if gs_data.microgames_completed == 15 && gs_data.lives > 0 {
                        events.push(GameEvents::GameWon);
                    }
                    if gs_data.lives <= 0 {
                        events.push(GameEvents::GameLost);
                    }
                    MicrogameState::TransIn(0.75)
                }
            }
        }

        return Ok(());
    } else {
        return Err(());
    }
}

fn lerp(f: f32, t: f32, d: f32) -> f32 {
    t * d + f * (1.0 - d)
}

// Scuffed ass function for easing towards the middle
fn anti_easing(f: f32) -> f32 {
    if f > 0.5 {
        ((f - 0.5) * 1.4).powf(2.0) + 0.5
    } else {
        0.5 - ((0.5 - f) * 1.4).powf(2.0)
    }
}

pub fn draw(game_data: &GameState) -> Result<(), ()> {
    if let ActiveState::InGame(gs_data) = &game_data.active_state {
        match &gs_data.current_microgame {
            Microgames::AlwaysWin(d) => always_win::draw(d),
            Microgames::Pipes(d) => pipes::draw(d, &game_data.textures),
            Microgames::Combo(d) => combo::draw(d, &game_data.textures),
            Microgames::Course(d) => course::draw(d, &game_data.textures),
            Microgames::Sweep(d) => sweep::draw(d, &game_data.textures),
            Microgames::Asteroids(d) => asteroids::draw(d, &game_data.textures),
            Microgames::Imposter(d) => imposter::draw(d, &game_data.textures),
            Microgames::Swap(d) => swap::draw(d, &game_data.textures),
            Microgames::Crank(d) => crank::draw(d, &game_data.textures),
            _ => {}
        }

        // Particles
        let maxtime: f32 = if gs_data.microgames_completed > 9 {
            3.0
        } else if gs_data.microgames_completed > 4 {
            4.0
        } else {
            5.0
        };

        // Draw UI
        if let MicrogameState::InMicrogame(t) = gs_data.microgame_state {
            draw_rectangle(30.0, 560.0, 900.0 * (t / maxtime), 10.0, YELLOW);
        } else {
            let r_door = game_data.textures.get("right_door").unwrap();
            let l_door = game_data.textures.get("left_door").unwrap();

            match gs_data.microgame_state {
                MicrogameState::TransIn(t) => {
                    let raw_progress = clamp((0.5 - t) * 2.0, 0.0, 1.0);
                    draw_texture(
                        l_door,
                        lerp(0.0, -500.0, raw_progress * raw_progress),
                        0.0,
                        WHITE,
                    );
                    draw_texture(
                        r_door,
                        lerp(462.0, 1000.0, raw_progress * raw_progress),
                        0.0,
                        WHITE,
                    );
                }
                MicrogameState::TransOut(t) => {
                    let raw_progress = clamp((0.7 - t) * (1.0 / 0.7), 0.0, 1.0);
                    draw_texture(
                        l_door,
                        lerp(-500.0, 0.0, raw_progress * raw_progress),
                        0.0,
                        WHITE,
                    );
                    draw_texture(
                        r_door,
                        lerp(1000.0, 462.0, raw_progress * raw_progress),
                        0.0,
                        WHITE,
                    );
                    match gs_data.end_plate {
                        EndPlateState::WonMG(s) => {
                            draw_texture(
                                game_data.textures.get("good_1").unwrap(),
                                380.0,
                                -150.0 + 750.0 * anti_easing(raw_progress),
                                WHITE,
                            );
                        }
                        _ => {}
                    };
                }
                _ => {}
            };
        }

        let heart = game_data.textures.get("heart").unwrap();
        for i in 0..gs_data.lives {
            draw_texture(heart, 16.0 + 80.0 * i as f32, 16.0, WHITE);
        }

        return Ok(());
    } else {
        return Err(());
    }
}
