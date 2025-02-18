use std::collections::HashMap;

use macroquad::prelude::*;

use crate::game::{FrameInput, MousePressState};

struct Target {
    range: (Vec2, Vec2),
    clicked: bool,
}

impl Target {
    fn new(min: Vec2, max: Vec2) -> Self {
        Target {
            range: (min, max),
            clicked: false,
        }
    }
}

pub struct Data {
    targets: Vec<Target>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            targets: vec![
                Target::new(Vec2::new(276.0, 110.0), Vec2::new(602.0, 279.0)),
                Target::new(Vec2::new(38.0, 366.0), Vec2::new(382.0, 546.0)),
                Target::new(Vec2::new(664.0, 306.0), Vec2::new(933.0, 501.0)),
            ],
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    if input.mouse_state == MousePressState::JustPressed {
        for t in data.targets.iter_mut() {
            if input.mouse_position.x > t.range.0.x
                && input.mouse_position.x < t.range.1.x
                && input.mouse_position.y > t.range.0.y
                && input.mouse_position.y < t.range.1.y
            {
                t.clicked = true;
            }
        }
    }

    data.targets
        .iter()
        .map(|t| t.clicked)
        .reduce(|e, acc| e && acc)
        .unwrap()
}

pub fn draw(data: &Data, tex_map: &HashMap<&str, Texture2D>) {
    clear_background(WHITE);

    draw_texture(tex_map.get("pipes_bkgd").unwrap(), 0.0, 0.0, WHITE);

    /*for t in data.targets.iter() {
        if t.clicked {
            draw_rectangle(
                t.range.0.x,
                t.range.0.y,
                t.range.1.x - t.range.0.x,
                t.range.1.y - t.range.0.y,
                GREEN,
            );
        } else {
            draw_rectangle(
                t.range.0.x,
                t.range.0.y,
                t.range.1.x - t.range.0.x,
                t.range.1.y - t.range.0.y,
                RED,
            );
        }
    }*/

    if data.targets[0].clicked {
        draw_texture(tex_map.get("pipes_patch_1").unwrap(), 276.0, 110.0, WHITE);
    }
    if data.targets[1].clicked {
        draw_texture(tex_map.get("pipes_patch_2").unwrap(), 38.0, 366.0, WHITE);
    }
    if data.targets[2].clicked {
        draw_texture(tex_map.get("pipes_patch_3").unwrap(), 664.0, 306.0, WHITE);
    }
}
