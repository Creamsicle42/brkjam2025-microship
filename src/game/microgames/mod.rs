pub mod always_win;
pub mod pipes;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
    Pipes(pipes::Data),
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::Pipes(pipes::Data::default());
}
