use macroquad::{prelude::*, rand::gen_range};
pub mod always_win;
pub mod asteroids;
pub mod combo;
pub mod course;
pub mod imposter;
pub mod pipes;
pub mod swap;
pub mod sweep;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
    Pipes(pipes::Data),
    Combo(combo::Data),
    Course(course::Data),
    Sweep(sweep::Data),
    Asteroids(asteroids::Data),
    Imposter(imposter::Data),
    Swap(swap::Data),
}

pub fn gen_new_microgame() -> Microgames {
    //match gen_range(0, 5) {
    //    0 => Microgames::Pipes(pipes::Data::default()),
    //    1 => Microgames::Combo(combo::Data::default()),
    //    2 => Microgames::Course(course::Data::default()),
    //    3 => Microgames::Sweep(sweep::Data::default()),
    //    4 => Microgames::Asteroids(asteroids::Data::default()),
    //    _ => Microgames::AlwaysWin(always_win::Data {}),
    //}
    return Microgames::Swap(swap::Data::default());
}
