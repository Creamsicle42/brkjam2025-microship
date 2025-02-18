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

pub fn init_game_state() -> GameState {
    let mut textures: HashMap<&str, Texture2D> = HashMap::new();

    textures.insert(
        "right_door",
        Texture2D::from_file_with_format(include_bytes!("../assets/right_door.png"), None),
    );
    textures.insert(
        "left_door",
        Texture2D::from_file_with_format(include_bytes!("../assets/left_door.png"), None),
    );
    textures.insert(
        "pipes_bkgd",
        Texture2D::from_file_with_format(include_bytes!("../assets/pipes_bkgd.png"), None),
    );
    textures.insert(
        "pipes_patch_1",
        Texture2D::from_file_with_format(include_bytes!("../assets/pipes_patch_1.png"), None),
    );
    textures.insert(
        "pipes_patch_2",
        Texture2D::from_file_with_format(include_bytes!("../assets/pipes_patch_2.png"), None),
    );
    textures.insert(
        "pipes_patch_3",
        Texture2D::from_file_with_format(include_bytes!("../assets/pipes_patch_3.png"), None),
    );

    build_textures_atlas();

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
