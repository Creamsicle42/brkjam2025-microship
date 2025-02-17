pub mod always_win;
pub mod asteroids;
pub mod combo;
pub mod course;
pub mod pipes;
pub mod sweep;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
    Pipes(pipes::Data),
    Combo(combo::Data),
    Course(course::Data),
    Sweep(sweep::Data),
    Asteroids(asteroids::Data),
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::Asteroids(asteroids::Data::default());
}
