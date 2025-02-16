#![allow(dead_code, unused)]

pub struct GameState {}
pub struct FrameInput {}

pub fn gather_frame_input() -> FrameInput {
    todo!()
}

pub fn init_game_state() -> GameState {
    todo!()
}

pub fn update_game_state(
    state: &mut GameState,
    input: FrameInput,
    deltatime: f32,
) -> Result<(), ()> {
    todo!()
}

pub fn draw_game_state(state: &GameState) -> Result<(), ()> {
    todo!()
}
