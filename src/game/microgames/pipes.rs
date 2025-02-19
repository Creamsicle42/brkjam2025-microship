use std::collections::HashMap;

use macroquad::prelude::*;
use rand::gen_range;

use crate::game::{particles_draw, particles_update, FrameInput, MousePressState, Particle};

struct Target {
    range: (Vec2, Vec2),
    clicked: bool,
    timer: f32,
}

impl Target {
    fn new(min: Vec2, max: Vec2) -> Self {
        Target {
            range: (min, max),
            clicked: false,
            timer: gen_range(0.01, 0.05),
        }
    }
}

pub struct Data {
    targets: Vec<Target>,
    particles: Vec<Particle>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            targets: vec![
                Target::new(Vec2::new(276.0, 110.0), Vec2::new(602.0, 279.0)),
                Target::new(Vec2::new(38.0, 366.0), Vec2::new(382.0, 546.0)),
                Target::new(Vec2::new(664.0, 306.0), Vec2::new(933.0, 501.0)),
            ],
            particles: vec![],
        }
    }
}

pub fn update(data: &mut Data, input: FrameInput, delta: f32) -> bool {
    for t in data.targets.iter_mut() {
        if input.mouse_position.x > t.range.0.x
            && input.mouse_position.x < t.range.1.x
            && input.mouse_position.y > t.range.0.y
            && input.mouse_position.y < t.range.1.y
            && input.mouse_state == MousePressState::JustPressed
        {
            t.clicked = true;
        }
        t.timer -= delta;
        if t.timer <= 0.0 && !t.clicked {
            t.timer = gen_range(0.05, 0.1);
            data.particles.push(Particle {
                position: Vec2::new(
                    gen_range(t.range.0.x + 50.0, t.range.1.x - 50.0),
                    gen_range(t.range.0.y + 50.0, t.range.1.y - 50.0),
                ),
                velocity: Vec2::new(gen_range(-10.0, 10.0), gen_range(40.0, 60.0)),
                color: Color {
                    r: 0.7,
                    g: 0.8,
                    b: 1.0,
                    a: 1.0,
                },
                color_delta: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: -0.1,
                },
                rotation: 0.0,
                ang_velocity: 1.0,
                lifetime: 1.0,
            });
        }
    }

    particles_update(&mut data.particles, &delta);

    data.targets
        .iter()
        .map(|t| t.clicked)
        .reduce(|e, acc| e && acc)
        .unwrap()
}

pub fn draw(data: &Data, tex_map: &HashMap<&str, Texture2D>) {
    clear_background(WHITE);

    draw_texture(tex_map.get("pipes_bkgd").unwrap(), 0.0, 0.0, WHITE);
    particles_draw(&data.particles, tex_map.get("smoke_particle").unwrap());

    /*for t in data.targets.iter() {
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
    }*/

    if data.targets[0].clicked {
        draw_texture(tex_map.get("pipes_patch_1").unwrap(), 276.0, 110.0, WHITE);
    }
    if data.targets[1].clicked {
        draw_texture(tex_map.get("pipes_patch_2").unwrap(), 38.0, 366.0, WHITE);
    }
    if data.targets[2].clicked {
        draw_texture(tex_map.get("pipes_patch_3").unwrap(), 664.0, 306.0, WHITE);
    }
}
