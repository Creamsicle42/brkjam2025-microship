use macroquad::prelude::*;

use super::{ActiveState, FrameInput, GameEvents, GameState, MousePressState};

pub struct MainMenuData {
    play_button_hover: bool,
    mouse_pos: Vec2,
}

impl Default for MainMenuData {
    fn default() -> Self {
        MainMenuData {
            play_button_hover: false,
            mouse_pos: Vec2::ZERO,
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
        if 0.0 < input.mouse_position.x
            && input.mouse_position.x < 100.0
            && 0.0 < input.mouse_position.y
            && input.mouse_position.y < 100.0
        {
            mm_data.play_button_hover = true;
        } else {
            mm_data.play_button_hover = false;
        }

        mm_data.mouse_pos = input.mouse_position.clone();

        if input.mouse_state == MousePressState::JustPressed && mm_data.play_button_hover {
            events.push(GameEvents::StartGameplay);
        }

        return Ok(());
    } else {
        return Err(());
    }
}

pub fn draw(game_data: &GameState) -> Result<(), ()> {
    if let ActiveState::MainMenu(mm_data) = &game_data.active_state {
        clear_background(WHITE);

        // Draw Play Button
        if mm_data.play_button_hover {
            draw_rectangle(0.0, 0.0, 100.0, 100.0, GREEN);
        } else {
            draw_rectangle(0.0, 0.0, 100.0, 100.0, RED);
        }

        draw_text(
            format!("{}, {}", mm_data.mouse_pos.x, mm_data.mouse_pos.y).as_str(),
            10.0,
            40.0,
            32.0,
            BLACK,
        );

        return Ok(());
    } else {
        return Err(());
    }
}
