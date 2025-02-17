use crate::game::{FrameInput, MousePressState};
use macroquad::prelude::*;
use rand::gen_range;

pub struct Data {
    asteroid_pos: Vec2,
    asteroid_vel: Vec2,
    lazer_pos: f32,
    lazer_vel: f32,
    did_shoot: bool,
    did_hit: bool,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            asteroid_pos: Vec2::new(gen_range(300.0, 850.0), gen_range(150.0, 650.0)),
            asteroid_vel: Vec2::new(gen_range(-10.0, 10.0), gen_range(-10.0, 10.0)),
            lazer_pos: 200.0,
            lazer_vel: 500.0,
            did_shoot: false,
            did_hit: false,
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    data.lazer_pos += data.lazer_vel * delta;
    if data.lazer_pos > 650.0 {
        data.lazer_pos = 650.0;
        data.lazer_vel *= -1.0;
    }
    if data.lazer_pos < 100.0 {
        data.lazer_pos = 100.0;
        data.lazer_vel *= -1.0;
    }

    data.asteroid_pos += data.asteroid_vel * delta;

    if input.mouse_state == MousePressState::JustPressed && !data.did_shoot {
        data.did_shoot = true;
        data.lazer_vel = 0.0;

        if (data.lazer_pos - data.asteroid_pos.y).abs() < 60.0 {
            data.did_hit = true;
        }
    }

    data.did_hit
}

pub fn draw(data: &Data) {
    clear_background(WHITE);
    if !data.did_shoot {
        draw_rectangle(0.0, data.lazer_pos - 5.0, 1000.0, 10.0, PINK);
    }
    if !data.did_hit {
        draw_rectangle(
            data.asteroid_pos.x - 50.0,
            data.asteroid_pos.y - 50.0,
            100.0,
            100.0,
            RED,
        );
    }

    draw_rectangle(
        0.0,
        data.lazer_pos - 25.0,
        50.0,
        50.0,
        if data.did_hit { GREEN } else { RED },
    );
}
