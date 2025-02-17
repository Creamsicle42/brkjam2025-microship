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

pub fn draw(data: &Data) {
    clear_background(WHITE);
    for i in 0..4 {
        draw_rectangle(
            SEL_AREAS[i].0.x,
            SEL_AREAS[i].0.y,
            SEL_AREAS[i].1.x - SEL_AREAS[i].0.x,
            SEL_AREAS[i].1.y - SEL_AREAS[i].0.y,
            if data.imposter_selected == i as i8 {
                GREEN
            } else {
                RED
            },
        );
        if data.imposter_hilighted == i as i8 && data.imposter_selected == -1 {
            draw_rectangle_lines(
                SEL_AREAS[i].0.x,
                SEL_AREAS[i].0.y,
                SEL_AREAS[i].1.x - SEL_AREAS[i].0.x,
                SEL_AREAS[i].1.y - SEL_AREAS[i].0.y,
                16.0,
                GREEN,
            );
        }
        if data.imposter_slot == i as u8 {
            draw_text("IMPOSTER", SEL_AREAS[i].0.x, SEL_AREAS[i].0.y, 16.0, BLACK);
        }
    }
}
