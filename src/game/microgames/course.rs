use std::collections::HashMap;

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
        let mut vals: Vec<i8> = vec![0, 1, 3, 4];
        vals.shuffle();
        Data {
            hovering_left: false,
            hovering_right: false,
            course: *vals.first().unwrap(),
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    data.hovering_right = (input.mouse_position.x > 100.0
        && input.mouse_position.x < 350.0
        && input.mouse_position.y > 100.0
        && input.mouse_position.y < 270.0);

    data.hovering_left = (input.mouse_position.x > 100.0
        && input.mouse_position.x < 350.0
        && input.mouse_position.y > 330.0
        && input.mouse_position.y < 510.0);

    if input.mouse_state == MousePressState::JustPressed {
        if data.hovering_right {
            data.course += 1;
        } else if data.hovering_left {
            data.course -= 1;
        }
        data.course = clamp(data.course, 0, 4);
    }

    data.course == 2
}

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    clear_background(WHITE);
    draw_texture(textures.get("course_bkgd").unwrap(), 0.0, 0.0, WHITE);
    draw_texture(
        textures
            .get(format!("course_temp_{}", data.course).as_str())
            .unwrap(),
        411.0,
        232.0,
        WHITE,
    );
}
