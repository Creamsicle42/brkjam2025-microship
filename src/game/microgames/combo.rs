use std::{iter::zip, usize};

use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;
use rand::ChooseRandom;

pub struct Button {
    range: (Vec2, Vec2),
    pressed: bool,
    order: u8,
}

pub struct Data {
    buttons: Vec<Button>,
    button_on: u8,
}

impl Default for Data {
    fn default() -> Self {
        let mut b_order: Vec<u8> = (0..4).collect();
        b_order.shuffle();

        let buttons: Vec<Button> = zip((0..4), b_order)
            .map(|(bnum, order)| Button {
                range: (
                    Vec2::new(116.0 + 116.0 * bnum as f32, 116.0),
                    Vec2::new(216.0 + 116.0 * bnum as f32, 216.0),
                ),
                pressed: false,
                order,
            })
            .collect();

        Data {
            buttons,
            button_on: 0,
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    let button_pressed: i8 = if input.mouse_state != MousePressState::JustPressed {
        -1
    } else {
        let mut b_pressed: i8 = -1;
        for i in (0..4) {
            let button: &Button = &data.buttons[i];
            if input.mouse_position.x > button.range.0.x
                && input.mouse_position.x < button.range.1.x
                && input.mouse_position.y > button.range.0.y
                && input.mouse_position.y < button.range.1.y
            {
                b_pressed = i as i8;
                break;
            }
        }
        b_pressed as i8
    };

    if button_pressed != -1 && !data.buttons[button_pressed as usize].pressed {
        if data.button_on == data.buttons[button_pressed as usize].order {
            data.button_on += 1;
            data.buttons[button_pressed as usize].pressed = true;
        } else {
            for button in data.buttons.iter_mut() {
                button.pressed = false;
            }
            data.button_on = 0;
        }
    }

    data.buttons
        .iter()
        .map(|b| b.pressed)
        .reduce(|acc, e| acc && e)
        .unwrap()
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
    for button in data.buttons.iter() {
        if button.pressed {
            draw_rectangle(
                button.range.0.x,
                button.range.0.y,
                button.range.1.x - button.range.0.x,
                button.range.1.y - button.range.0.y,
                GREEN,
            );
        } else {
            draw_rectangle(
                button.range.0.x,
                button.range.0.y,
                button.range.1.x - button.range.0.x,
                button.range.1.y - button.range.0.y,
                RED,
            );
        }
        draw_text(
            format!("{}", button.order).as_str(),
            button.range.0.x + 50.0,
            button.range.0.y + 50.0,
            32.0,
            BLACK,
        );
    }
}
