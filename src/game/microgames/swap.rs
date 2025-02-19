use std::{collections::HashMap, usize};

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
                Vec2::new(30.0 + i as f32 * 310.0, 80.0),
                Vec2::new(320.0 + i as f32 * 310.0, 650.0),
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

    data.order[0] < data.order[1] && data.order[1] < data.order[2]
}

const BOX_OFFSETS: [f32; 3] = [233.0, 152.0, 80.0];

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    draw_texture(textures.get("swap_bkgd").unwrap(), 0.0, 0.0, WHITE);
    for (height, index) in data.order.iter().zip(0..3) {
        draw_texture(
            textures
                .get(format!("swap_can_{}", height).as_str())
                .unwrap(),
            30.0 + 310.0 * index as f32,
            BOX_OFFSETS[*height as usize] - if (data.selected == index) { 40.0 } else { 0.0 },
            WHITE,
        );
    }
}
