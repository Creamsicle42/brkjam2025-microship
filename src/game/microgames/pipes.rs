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
                Target::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)),
                Target::new(Vec2::new(200.0, 100.0), Vec2::new(300.0, 200.0)),
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

pub fn draw(data: &Data) {
    clear_background(WHITE);

    for t in data.targets.iter() {
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
    }
}
