pub mod always_win;
pub mod combo;
pub mod course;
pub mod pipes;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
    Pipes(pipes::Data),
    Combo(combo::Data),
    Course(course::Data),
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::Course(course::Data::default());
}
