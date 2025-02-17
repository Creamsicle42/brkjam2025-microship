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
    let left_handle_bound = 100.0 + 700.0 * data.progress;

    let mouse_on_handle = (input.mouse_position.x > left_handle_bound
        && input.mouse_position.x < left_handle_bound + 100.0
        && input.mouse_position.y > 600.0
        && input.mouse_position.y < 700.0);

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
        delta * 4.0,
    );

    if data.is_dragging {
        data.progress += drag_delta;
        data.drag_pos = input.mouse_position.x;
    }

    data.progress = clamp(data.progress, 0.0, 1.0);

    data.progress >= 1.0
}

pub fn draw(data: &Data) {
    clear_background(WHITE);

    draw_rectangle(
        100.0 + 700.0 * data.progress,
        600.0,
        100.0,
        100.0,
        if data.progress >= 1.0 { GREEN } else { RED },
    );
    if data.is_dragging {
        draw_rectangle_lines(
            100.0 + 700.0 * data.progress,
            600.0,
            100.0,
            100.0,
            16.0,
            GREEN,
        );
    }
}
