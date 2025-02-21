use macroquad::prelude::*;

use super::{ActiveState, FrameInput, GameEvents, GameState, MousePressState};

#[derive(Debug, PartialEq)]
enum TransState {
    None,
    Out(f32),
}

pub struct MainMenuData {
    play_button_hover: bool,
    mouse_pos: Vec2,
    t_state: TransState,
}

impl Default for MainMenuData {
    fn default() -> Self {
        MainMenuData {
            play_button_hover: false,
            mouse_pos: Vec2::ZERO,
            t_state: TransState::None,
        }
    }
}

pub fn update(
    game_data: &mut GameState,
    input: FrameInput,
    delta: f32,
    events: &mut Vec<GameEvents>,
) -> Result<(), ()> {
    if let ActiveState::MainMenu(mm_data) = &mut game_data.active_state {
        // Check if player is over play button
        if 580.0 < input.mouse_position.x
            && input.mouse_position.x < 910.0
            && 333.0 < input.mouse_position.y
            && input.mouse_position.y < 490.0
        {
            mm_data.play_button_hover = true;
        } else {
            mm_data.play_button_hover = false;
        }

        mm_data.mouse_pos = input.mouse_position.clone();

        if input.mouse_state == MousePressState::JustPressed
            && mm_data.play_button_hover
            && mm_data.t_state == TransState::None
        {
            mm_data.t_state = TransState::Out(0.5);
        }

        mm_data.t_state = match &mm_data.t_state {
            TransState::Out(t) => {
                if *t <= 0.0 {
                    events.push(GameEvents::StartGameplay);
                };
                TransState::Out(t - delta)
            }
            TransState::None => TransState::None,
        };

        return Ok(());
    } else {
        return Err(());
    }
}
fn lerp(f: f32, t: f32, d: f32) -> f32 {
    t * d + f * (1.0 - d)
}

pub fn draw(game_data: &GameState) -> Result<(), ()> {
    if let ActiveState::MainMenu(mm_data) = &game_data.active_state {
        draw_texture(
            game_data.textures.get("title_screen_bkgd").unwrap(),
            0.0,
            0.0,
            WHITE,
        );

        // Draw Play Button
        if mm_data.play_button_hover {
            draw_texture(
                game_data.textures.get("title_screen_button_green").unwrap(),
                580.0,
                333.0,
                WHITE,
            );
        } else {
            draw_texture(
                game_data.textures.get("title_screen_button_red").unwrap(),
                580.0,
                333.0,
                WHITE,
            );
        }

        match mm_data.t_state {
            TransState::Out(t) => {
                let raw_progress = clamp((0.5 - t) * 2.0, 0.0, 1.0);
                let r_door = game_data.textures.get("right_door").unwrap();
                let l_door = game_data.textures.get("left_door").unwrap();
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
            }
            _ => {}
        }

        return Ok(());
    } else {
        return Err(());
    }
}
