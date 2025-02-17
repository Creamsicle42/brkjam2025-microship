pub mod always_win;

pub enum Microgames {
    Dummy,
    AlwaysWin(always_win::Data),
}

pub fn gen_new_microgame() -> Microgames {
    return Microgames::AlwaysWin(always_win::Data {});
}
