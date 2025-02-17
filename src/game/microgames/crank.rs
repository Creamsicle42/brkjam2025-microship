use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;

pub struct Data {
    angle: f32,
    is_draging: bool,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            angle: 1.0,
            is_draging: false,
        }
    }
}

const CRANK_CENTER: Vec2 = Vec2::new(300.0, 400.0);
const CRANK_LENGTH: f32 = 200.0;

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    let crank_off = Vec2::from_angle(data.angle) * CRANK_LENGTH;

    let player_mouse_dist = input.mouse_position.distance(crank_off + CRANK_CENTER);

    if player_mouse_dist > 80.0 || input.mouse_state == MousePressState::NotPressed {
        data.is_draging = false;
    }

    if player_mouse_dist < 50.0 && input.mouse_state == MousePressState::JustPressed {
        data.is_draging = true;
    }

    let angle_to_mouse = crank_off.angle_between(input.mouse_position - CRANK_CENTER);

    if data.is_draging {
        data.angle += clamp(angle_to_mouse, 0.0, delta * 10.0);
    }
    data.angle = data.angle.clamp(1.0, 7.0);

    data.angle >= 7.0
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
    let crank_off = Vec2::from_angle(data.angle) * CRANK_LENGTH;
    draw_circle(
        crank_off.x + CRANK_CENTER.x,
        crank_off.y + CRANK_CENTER.y,
        50.0,
        if data.is_draging { GREEN } else { RED },
    );
    draw_text(
        format!("{}", data.angle).as_str(),
        CRANK_CENTER.x,
        CRANK_CENTER.y,
        32.0,
        BLACK,
    );
}
