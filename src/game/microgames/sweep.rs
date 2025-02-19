use std::collections::HashMap;

use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;

pub struct Data {
    progress: f32,
    is_dragging: bool,
    drag_pos: f32,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            progress: 0.0,
            is_dragging: false,
            drag_pos: 0.0,
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    let left_handle_bound = 20.0 + 800.0 * data.progress;

    let mouse_on_handle = (input.mouse_position.x > left_handle_bound
        && input.mouse_position.x < left_handle_bound + 100.0
        && input.mouse_position.y > 420.0
        && input.mouse_position.y < 590.0);

    if mouse_on_handle && input.mouse_state == MousePressState::JustPressed {
        data.is_dragging = true;
        data.drag_pos = input.mouse_position.x;
    }

    if !mouse_on_handle || input.mouse_state == MousePressState::NotPressed {
        data.is_dragging = false;
    }

    let drag_delta = clamp(
        (input.mouse_position.x - data.drag_pos) / 700.0,
        0.0,
        delta * 1.0,
    );

    if data.is_dragging {
        data.progress += drag_delta;
        data.drag_pos = input.mouse_position.x;
    }

    data.progress = clamp(data.progress, 0.0, 1.0);

    data.progress >= 1.0
}

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    draw_texture(textures.get("sweep_bkgd").unwrap(), 0.0, 0.0, WHITE);

    if data.progress >= 1.0 {
        draw_texture(textures.get("sweep_frame_3").unwrap(), 294.0, 24.0, WHITE);
    } else if data.progress >= 0.5 {
        draw_texture(textures.get("sweep_frame_2").unwrap(), 335.0, 25.0, WHITE);
    } else {
        draw_texture(textures.get("sweep_frame_1").unwrap(), 217.0, 21.0, WHITE);
    }

    let left_handle_bound = 20.0 + 800.0 * data.progress;
    draw_texture(
        textures.get("sweep_handle").unwrap(),
        left_handle_bound,
        420.0,
        WHITE,
    );
}
