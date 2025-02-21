#![allow(dead_code, unused)]
use std::{collections::HashMap, usize};

use gameplay::InGameData;
use macroquad::prelude::*;
use main_menu::MainMenuData;

mod gameplay;
mod main_menu;
mod microgames;

#[derive(Debug, PartialEq)]
pub enum GameEvents {
    StartGameplay,
    GameWon,
    GameLost,
    MainMenuReturn,
}

enum ActiveState {
    MainMenu(MainMenuData),
    InGame(InGameData),
    WinScreen,
    LoseScreen,
}

pub struct GameState {
    active_state: ActiveState,
    textures: HashMap<&'static str, Texture2D>,
    smoke_particle: Texture2D,
}

#[derive(Debug, PartialEq)]
enum MousePressState {
    NotPressed,
    JustPressed,
    Pressed,
}

pub struct FrameInput {
    mouse_position: Vec2,
    mouse_state: MousePressState,
}

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub ang_velocity: f32,
    pub lifetime: f32,
    pub color: Color,
    pub color_delta: Color,
}

pub fn particles_update(particles: &mut Vec<Particle>, delta: &f32) {
    let mut remove_queue: Vec<usize> = vec![];
    let d = *delta;
    for (p, i) in particles.iter_mut().zip(0..) {
        p.position += p.velocity * d;
        p.rotation += p.ang_velocity * d;
        p.color.r += p.color_delta.r * *delta;
        p.color.g += p.color_delta.g * *delta;
        p.color.b += p.color_delta.b * *delta;
        p.color.a = p.color.a - (1.0 * d);
        p.lifetime -= *delta;
        if p.lifetime <= 0.0 {
            remove_queue.push(i);
        }
    }
    for i in remove_queue.into_iter().rev() {
        particles.swap_remove(i);
    }
}

pub fn particles_draw(particles: &Vec<Particle>, tex: &Texture2D) {
    let w = tex.width() / 2.0;
    let h = tex.height() / 2.0;
    for p in particles.iter() {
        draw_texture_ex(
            tex,
            p.position.x - w,
            p.position.y - h,
            p.color,
            DrawTextureParams {
                rotation: p.rotation,
                ..Default::default()
            },
        );
    }
}

pub fn vec2_in_range(v: &Vec2, r: &(Vec2, Vec2)) -> bool {
    v.x > r.0.x && v.x < r.1.x && v.y > r.0.y && v.y < r.1.y
}

pub fn gather_frame_input() -> FrameInput {
    FrameInput {
        mouse_position: {
            let (x, y) = mouse_position();
            Vec2 { x, y }
        },
        mouse_state: {
            if is_mouse_button_pressed(MouseButton::Left) {
                MousePressState::JustPressed
            } else if is_mouse_button_down(MouseButton::Left) {
                MousePressState::Pressed
            } else {
                MousePressState::NotPressed
            }
        },
    }
}

macro_rules! include_texture {
    ($map: ident, $tex: literal, $path: literal) => {
        $map.insert(
            $tex,
            Image::from_file_with_format(include_bytes!($path), Some(ImageFormat::Png)).unwrap(),
        );
    };
}

pub fn get_texture_images() -> HashMap<&'static str, Image> {
    let mut textures: HashMap<&str, Image> = HashMap::new();
    include_texture!(textures, "right_door", "../assets/right_door.png");
    include_texture!(textures, "left_door", "../assets/left_door.png");
    include_texture!(textures, "pipes_bkgd", "../assets/pipes_bkgd.png");
    include_texture!(textures, "pipes_patch_1", "../assets/pipes_patch_1.png");
    include_texture!(textures, "pipes_patch_2", "../assets/pipes_patch_2.png");
    include_texture!(textures, "pipes_patch_3", "../assets/pipes_patch_3.png");
    include_texture!(textures, "combo_bkgd", "../assets/combo_bkgd.png");
    include_texture!(textures, "combo_red_1", "../assets/combo_red_1.png");
    include_texture!(textures, "combo_red_2", "../assets/combo_red_2.png");
    include_texture!(textures, "combo_red_3", "../assets/combo_red_3.png");
    include_texture!(textures, "combo_red_4", "../assets/combo_red_4.png");
    include_texture!(textures, "combo_green_1", "../assets/combo_green_1.png");
    include_texture!(textures, "combo_green_2", "../assets/combo_green_2.png");
    include_texture!(textures, "combo_green_3", "../assets/combo_green_3.png");
    include_texture!(textures, "combo_green_4", "../assets/combo_green_4.png");

    include_texture!(textures, "course_bkgd", "../assets/course_bkgd.png");
    include_texture!(textures, "course_temp_0", "../assets/course_xlow.png");
    include_texture!(textures, "course_temp_1", "../assets/course_low.png");
    include_texture!(textures, "course_temp_2", "../assets/course_good.png");
    include_texture!(textures, "course_temp_3", "../assets/course_high.png");
    include_texture!(textures, "course_temp_4", "../assets/course_xhigh.png");

    include_texture!(textures, "swap_bkgd", "../assets/swap_bkgd.png");
    include_texture!(textures, "swap_can_0", "../assets/swap_small.png");
    include_texture!(textures, "swap_can_1", "../assets/swap_medium.png");
    include_texture!(textures, "swap_can_2", "../assets/swap_tall.png");

    include_texture!(textures, "imposter_bkgd", "../assets/imposter_bkgd.png");
    include_texture!(textures, "imposter_human", "../assets/imposter_human.png");
    include_texture!(
        textures,
        "imposter_human_bloody",
        "../assets/imposter_human_bloody.png"
    );
    include_texture!(
        textures,
        "imposter_imposter",
        "../assets/imposter_imposter.png"
    );
    include_texture!(
        textures,
        "imposter_imposter_bloody",
        "../assets/imposter_imposter_bloody.png"
    );

    include_texture!(textures, "crank_bkgd", "../assets/crank_bkgd.png");
    include_texture!(textures, "crank_door", "../assets/crank_door.png");
    include_texture!(textures, "crank_handle", "../assets/crank_handle.png");

    include_texture!(textures, "asteroids_bkgd", "../assets/asteroids_bkgd.png");
    include_texture!(
        textures,
        "asteroids_lazer_green",
        "../assets/asteroids_green_lazer.png"
    );
    include_texture!(
        textures,
        "asteroids_lazer_red",
        "../assets/asteroids_red_lazer.png"
    );
    include_texture!(
        textures,
        "asteroids_asteroid",
        "../assets/asteroids_asteroid.png"
    );
    include_texture!(textures, "asteroids_chunk", "../assets/asteroids_chunk.png");
    include_texture!(textures, "asteroids_beam", "../assets/asteroids_beam.png");

    include_texture!(textures, "sweep_bkgd", "../assets/sweep_bkgnd.png");
    include_texture!(textures, "sweep_handle", "../assets/sweep_handle.png");
    include_texture!(textures, "sweep_frame_1", "../assets/sweep_frame_1.png");
    include_texture!(textures, "sweep_frame_2", "../assets/sweep_frame_2.png");
    include_texture!(textures, "sweep_frame_3", "../assets/sweep_frame_3.png");

    include_texture!(textures, "smoke_particle", "../assets/smoke_particle.png");

    include_texture!(
        textures,
        "title_screen_bkgd",
        "../assets/title_screen_bkgd.png"
    );
    include_texture!(
        textures,
        "title_screen_button_red",
        "../assets/title_screen_button_red.png"
    );
    include_texture!(
        textures,
        "title_screen_button_green",
        "../assets/title_screen_button_green.png"
    );

    return textures;
}

pub fn init_game_state(images: HashMap<&'static str, Image>) -> GameState {
    let mut textures: HashMap<&str, Texture2D> = HashMap::new();

    for (id, img) in images.iter() {
        textures.insert(id, Texture2D::from_image(img));
    }

    //build_textures_atlas();

    GameState {
        smoke_particle: Texture2D::from_file_with_format(
            include_bytes!("../assets/smoke_particle.png"),
            None,
        ),
        textures,
        active_state: ActiveState::MainMenu(MainMenuData::default()),
    }
}

pub fn update_game_state(
    state: &mut GameState,
    input: FrameInput,
    deltatime: f32,
) -> Result<(), ()> {
    let mut events: Vec<GameEvents> = vec![];
    let out = match &state.active_state {
        ActiveState::MainMenu(_) => main_menu::update(state, input, deltatime, &mut events),
        ActiveState::InGame(_) => gameplay::update(state, input, deltatime, &mut events),
        ActiveState::WinScreen | ActiveState::LoseScreen => {
            if input.mouse_state == MousePressState::JustPressed {
                events.push(GameEvents::MainMenuReturn);
            }
            Ok(())
        }
        _ => todo!(),
    };

    if events.contains(&GameEvents::StartGameplay) {
        let mut new_game_state = InGameData::default();
        state.active_state = ActiveState::InGame(new_game_state);
    };

    if events.contains(&GameEvents::GameWon) {
        state.active_state = ActiveState::WinScreen;
    }

    if events.contains(&GameEvents::GameLost) {
        state.active_state = ActiveState::LoseScreen;
    }

    if events.contains(&GameEvents::MainMenuReturn) {
        state.active_state = ActiveState::MainMenu(MainMenuData::default());
    }

    return out;
}

pub fn draw_game_state(state: &GameState) -> Result<(), ()> {
    match &state.active_state {
        ActiveState::MainMenu(_) => main_menu::draw(state),
        ActiveState::InGame(_) => gameplay::draw(state),
        ActiveState::LoseScreen => {
            clear_background(WHITE);
            draw_text("You lose", 32.0, 48.0, 32.0, BLACK);
            Ok(())
        }
        ActiveState::WinScreen => {
            clear_background(WHITE);
            draw_text("You win", 32.0, 48.0, 32.0, BLACK);
            Ok(())
        }
        _ => todo!(),
    }
}
