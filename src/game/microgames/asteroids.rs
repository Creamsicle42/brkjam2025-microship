use std::collections::HashMap;

use crate::game::{particles_draw, particles_update, FrameInput, MousePressState, Particle};
use macroquad::prelude::*;
use rand::gen_range;

pub struct Data {
    asteroid_pos: Vec2,
    asteroid_vel: Vec2,
    lazer_pos: f32,
    lazer_vel: f32,
    did_shoot: bool,
    did_hit: bool,
    lazer_cooltime: f32,
    chunks_pos: Vec<Vec2>,
    chunks_vel: Vec<Vec2>,
    particles: Vec<Particle>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            asteroid_pos: Vec2::new(gen_range(300.0, 850.0), gen_range(150.0, 490.0)),
            asteroid_vel: Vec2::new(gen_range(-10.0, 10.0), gen_range(-10.0, 10.0)),
            lazer_pos: 200.0,
            lazer_vel: 500.0,
            did_shoot: false,
            did_hit: false,
            lazer_cooltime: 0.0,
            chunks_pos: vec![],
            chunks_vel: vec![],
            particles: vec![],
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    data.lazer_pos += data.lazer_vel * delta;
    if data.lazer_pos > 540.0 {
        data.lazer_pos = 540.0;
        data.lazer_vel *= -1.0;
    }
    if data.lazer_pos < 100.0 {
        data.lazer_pos = 100.0;
        data.lazer_vel *= -1.0;
    }

    data.asteroid_pos += data.asteroid_vel * delta;
    data.lazer_cooltime -= delta;

    if input.mouse_state == MousePressState::JustPressed && !data.did_shoot {
        data.did_shoot = true;
        data.lazer_vel = 0.0;
        data.lazer_cooltime = 0.25;

        if (data.lazer_pos - data.asteroid_pos.y).abs() < 150.0 {
            data.did_hit = true;
            for _ in 0..6 {
                data.chunks_pos.push(
                    data.asteroid_pos + Vec2::new(gen_range(-50.0, 50.0), gen_range(-50.0, 50.0)),
                );
                data.chunks_vel
                    .push(Vec2::new(gen_range(-50.0, 50.0), gen_range(-50.0, 50.0)));
            }
            for _ in 0..32 {
                data.particles.push(Particle {
                    position: data.asteroid_pos
                        + Vec2::new(gen_range(-50.0, 50.0), gen_range(-50.0, 50.0)),
                    velocity: Vec2::new(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
                    rotation: gen_range(0.0, 6.2),
                    ang_velocity: gen_range(-1.0, 1.0),
                    color: Color {
                        r: 0.8,
                        g: 0.4,
                        b: 0.4,
                        a: 0.5,
                    },
                    color_delta: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: -0.1,
                    },
                    lifetime: 0.5,
                });
            }
        }
    }

    particles_update(&mut data.particles, &delta);

    for (pos, vel) in data.chunks_pos.iter_mut().zip(data.chunks_vel.iter_mut()) {
        *pos += *vel * delta;
    }

    data.did_hit
}

pub fn draw(data: &Data, textures: &HashMap<&str, Texture2D>) {
    draw_texture(textures.get("asteroids_bkgd").unwrap(), 0.0, 0.0, WHITE);
    if !data.did_shoot {
        draw_rectangle(0.0, data.lazer_pos - 5.0, 1000.0, 10.0, PINK);
    }
    if !data.did_hit {
        draw_texture(
            textures.get("asteroids_asteroid").unwrap(),
            data.asteroid_pos.x - 150.0,
            data.asteroid_pos.y - 150.0,
            WHITE,
        );
    }

    draw_texture(
        textures
            .get(if data.did_shoot {
                "asteroids_lazer_red"
            } else {
                "asteroids_lazer_green"
            })
            .unwrap(),
        0.0,
        data.lazer_pos - 70.0,
        WHITE,
    );

    if data.lazer_cooltime > 0.0 {
        draw_texture(
            textures.get("asteroids_beam").unwrap(),
            105.0,
            data.lazer_pos - 30.0,
            WHITE,
        );
    }

    for (pos, i) in data.chunks_pos.iter().zip(0..) {
        draw_texture_ex(
            textures.get("asteroids_chunk").unwrap(),
            pos.x - 50.0,
            pos.y - 50.0,
            WHITE,
            DrawTextureParams {
                rotation: i as f32,
                ..Default::default()
            },
        );
    }

    particles_draw(&data.particles, textures.get("smoke_particle").unwrap());
}
