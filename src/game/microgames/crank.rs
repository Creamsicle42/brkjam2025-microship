use std::collections::HashMap;

use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;

pub struct Data {
    angle: f32,
    is_draging: bool,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            angle: -1.5,
            is_draging: false,
        }
    }
}

const CRANK_CENTER: Vec2 = Vec2::new(100.0, 700.0);
const CRANK_LENGTH: f32 = 380.0;

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
        data.angle += clamp(angle_to_mouse, 0.0, delta * 1.0);
    }
    data.angle = data.angle.clamp(-1.5, -0.5);

    data.angle >= -0.5
}

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    clear_background(BLACK);

    draw_texture(
        textures.get("crank_door").unwrap(),
        496.0,
        164.0 - 200.0 * ((data.angle + 1.5) / 1.0),
        WHITE,
    );
    draw_texture(textures.get("crank_bkgd").unwrap(), 0.0, 0.0, WHITE);

    let crank_draw_params = DrawTextureParams {
        rotation: data.angle,
        //pivot: Some(CRANK_CENTER),
        ..Default::default()
    };

    let crank_off = Vec2::from_angle(data.angle) * (CRANK_LENGTH - 165.0);

    draw_texture_ex(
        textures.get("crank_handle").unwrap(),
        crank_off.x + CRANK_CENTER.x - 200.0,
        crank_off.y + CRANK_CENTER.y - 72.0,
        WHITE,
        crank_draw_params,
    );
    /*draw_circle(
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
    );*/
}
