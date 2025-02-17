use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;
use rand::{gen_range, ChooseRandom};

pub struct Data {
    hovering_right: bool,
    hovering_left: bool,
    course: i8,
}

impl Default for Data {
    fn default() -> Self {
        let mut vals: Vec<i8> = vec![-3, -2, 2, 3];
        vals.shuffle();
        Data {
            hovering_left: false,
            hovering_right: false,
            course: *vals.first().unwrap(),
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    data.hovering_right = (input.mouse_position.x > 600.0
        && input.mouse_position.x < 950.0
        && input.mouse_position.y > 400.0
        && input.mouse_position.y < 700.0);

    data.hovering_left = (input.mouse_position.x > 50.0
        && input.mouse_position.x < 400.0
        && input.mouse_position.y > 400.0
        && input.mouse_position.y < 700.0);

    if input.mouse_state == MousePressState::JustPressed {
        if data.hovering_right {
            data.course += 1;
        } else if data.hovering_left {
            data.course -= 1;
        }
        data.course = clamp(data.course, -3, 3);
    }

    data.course == 0
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
    if data.hovering_right {
        draw_rectangle(600.0, 400.0, 350.0, 300.0, GREEN);
    } else {
        draw_rectangle(600.0, 400.0, 350.0, 300.0, RED);
    }
    if data.hovering_left {
        draw_rectangle(50.0, 400.0, 350.0, 300.0, GREEN);
    } else {
        draw_rectangle(50.0, 400.0, 350.0, 300.0, RED);
    }
    draw_rectangle(
        450.0 + 100.0 * data.course as f32,
        100.0,
        100.0,
        100.0,
        if data.course == 0 { GREEN } else { RED },
    );
}
