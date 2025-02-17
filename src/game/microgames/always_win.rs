use macroquad::prelude::*;

use crate::game::FrameInput;

pub struct Data {}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    true
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
}
