pub mod always_win;
pub mod combo;
pub mod pipes;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
    Pipes(pipes::Data),
    Combo(combo::Data),
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::Combo(combo::Data::default());
}
