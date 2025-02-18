use macroquad::{prelude::*, rand::gen_range};
use rand::ChooseRandom;
pub mod always_win;
pub mod asteroids;
pub mod combo;
pub mod course;
pub mod crank;
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
    Crank(crank::Data),
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
    return Microgames::Asteroids(asteroids::Data::default());
}

pub fn gen_microgame(game_id: u8) -> Microgames {
    return Microgames::Combo(combo::Data::default());
    match game_id {
        0 => Microgames::Pipes(pipes::Data::default()),
        1 => Microgames::Combo(combo::Data::default()),
        2 => Microgames::Course(course::Data::default()),
        3 => Microgames::Sweep(sweep::Data::default()),
        4 => Microgames::Asteroids(asteroids::Data::default()),
        5 => Microgames::Imposter(imposter::Data::default()),
        6 => Microgames::Crank(crank::Data::default()),
        7 => Microgames::Swap(swap::Data::default()),
        _ => Microgames::AlwaysWin(always_win::Data {}),
    }
}

pub fn gen_microgame_queue() -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let mut ran: Vec<u8> = (0..8).collect();
    ran.shuffle();
    for n in ran.iter().take(6) {
        out.push(*n);
    }
    ran.shuffle();
    for n in ran.iter().take(6) {
        out.push(*n);
    }
    ran.shuffle();
    for n in ran.iter().take(4) {
        out.push(*n);
    }
    return out;
}
