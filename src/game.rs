#![allow(dead_code, unused)]
use std::collections::HashMap;

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
            Texture2D::from_file_with_format(include_bytes!($path), None),
        );
    };
}

pub async fn init_game_state() -> GameState {
    let mut textures: HashMap<&str, Texture2D> = HashMap::new();

    include_texture!(textures, "right_door", "../assets/right_door.png");
    include_texture!(textures, "left_door", "../assets/left_door.png");
    include_texture!(textures, "pipes_bkgd", "../assets/pipes_bkgd.png");
    include_texture!(textures, "pipes_patch_1", "../assets/pipes_patch_1.png");
    include_texture!(textures, "pipes_patch_2", "../assets/pipes_patch_2.png");
    include_texture!(textures, "pipes_patch_3", "../assets/pipes_patch_3.png");
    textures.insert(
        "combo_bkgd",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_bkgd.png"), None),
    );
    textures.insert(
        "combo_red_1",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_red_1.png"), None),
    );
    textures.insert(
        "combo_red_2",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_red_2.png"), None),
    );
    textures.insert(
        "combo_red_3",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_red_3.png"), None),
    );
    textures.insert(
        "combo_red_4",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_red_4.png"), None),
    );
    textures.insert(
        "combo_green_1",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_green_1.png"), None),
    );
    textures.insert(
        "combo_green_2",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_green_2.png"), None),
    );
    textures.insert(
        "combo_green_3",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_green_3.png"), None),
    );
    textures.insert(
        "combo_green_4",
        Texture2D::from_file_with_format(include_bytes!("../assets/combo_green_4.png"), None),
    );

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

    //build_textures_atlas();

    GameState {
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
