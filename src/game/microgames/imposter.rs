use std::collections::HashMap;

use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;
use rand::gen_range;

pub struct Data {
    imposter_slot: u8,
    imposter_hilighted: i8,
    imposter_selected: i8,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            imposter_slot: gen_range(0, 4),
            imposter_hilighted: -1,
            imposter_selected: -1,
        }
    }
}

const SEL_AREAS: [(Vec2, Vec2); 4] = [
    (Vec2::new(50.0, 100.0), Vec2::new(200.0, 650.0)),
    (Vec2::new(300.0, 100.0), Vec2::new(450.0, 650.0)),
    (Vec2::new(550.0, 100.0), Vec2::new(700.0, 650.0)),
    (Vec2::new(800.0, 100.0), Vec2::new(950.0, 650.0)),
];

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    let mut is_sel = false;
    for i in 0..4 {
        if (input.mouse_position.x > SEL_AREAS[i].0.x
            && input.mouse_position.x < SEL_AREAS[i].1.x
            && input.mouse_position.y > SEL_AREAS[i].0.y
            && input.mouse_position.y < SEL_AREAS[i].1.y)
        {
            is_sel = true;
            data.imposter_hilighted = i as i8;
        }
    }
    if !is_sel {
        data.imposter_hilighted = -1;
    }

    if input.mouse_state == MousePressState::JustPressed
        && data.imposter_hilighted != -1
        && data.imposter_hilighted != -1
    {
        data.imposter_selected = data.imposter_hilighted;
    }

    data.imposter_selected == data.imposter_slot as i8
}

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    draw_texture(textures.get("imposter_bkgd").unwrap(), 0.0, 0.0, WHITE);
    for i in 0..4 {
        let tex_id = match (data.imposter_selected == i, data.imposter_slot == i as u8) {
            (true, true) => "imposter_imposter_bloody",
            (true, false) => "imposter_human_bloody",
            (false, true) => "imposter_imposter",
            (false, false) => "imposter_human",
        };

        draw_texture(
            textures.get(tex_id).unwrap(),
            70.0 + 240.0 * i as f32,
            246.0,
            WHITE,
        );
    }
}
