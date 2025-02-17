use crate::game::{vec2_in_range, FrameInput, MousePressState};
use macroquad::prelude::*;
use rand::ChooseRandom;

pub struct Data {
    order: Vec<u8>,
    selected: i8,
}

impl Default for Data {
    fn default() -> Self {
        let mut order: Vec<u8> = (0..3).collect();
        order.shuffle();
        Data {
            order,
            selected: -1,
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    let mut hovered: i8 = -1;
    let mut click_boxes: Vec<(Vec2, Vec2)> = data
        .order
        .iter()
        .zip(0..3)
        .map(|(val, i)| {
            (
                Vec2::new(
                    100.0 + i as f32 * 150.0,
                    200.0 + 50.0 * *val as f32
                        - (if data.selected == i as i8 { 25.0 } else { 0.0 }),
                ),
                Vec2::new(
                    200.0 + i as f32 * 150.0,
                    650.0 - (if data.selected == i as i8 { 25.0 } else { 0.0 }),
                ),
            )
        })
        .collect();

    for (box_range, i) in click_boxes.iter().zip(0..click_boxes.len()) {
        if vec2_in_range(&input.mouse_position, box_range) {
            hovered = i as i8;
            //println!("Setting hover to {hovered}");
        }
    }

    if hovered != -1 && input.mouse_state == MousePressState::JustPressed {
        if hovered == data.selected {
            data.selected = -1;
        } else if data.selected != -1 {
            let temp = data.order[data.selected as usize];
            data.order[data.selected as usize] = data.order[hovered as usize];
            data.order[hovered as usize] = temp;
            data.selected = -1;
        } else {
            data.selected = hovered;
        }
    }

    data.order[0] > data.order[1] && data.order[1] > data.order[2]
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
    let mut click_boxes: Vec<(Vec2, Vec2)> = data
        .order
        .iter()
        .zip(0..3)
        .map(|(val, i)| {
            (
                Vec2::new(
                    100.0 + i as f32 * 150.0,
                    200.0 + 50.0 * *val as f32
                        - (if data.selected == i as i8 { 25.0 } else { 0.0 }),
                ),
                Vec2::new(
                    200.0 + i as f32 * 150.0,
                    650.0 - (if data.selected == i as i8 { 25.0 } else { 0.0 }),
                ),
            )
        })
        .collect();
    for (rl, br) in click_boxes.iter() {
        draw_rectangle(rl.x, rl.y, br.x - rl.x, br.y - rl.y, RED);
    }
}
