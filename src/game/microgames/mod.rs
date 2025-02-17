pub mod always_win;
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
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::Sweep(sweep::Data::default());
}
