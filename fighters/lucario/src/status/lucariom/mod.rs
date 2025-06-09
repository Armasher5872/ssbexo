use {
    exo_var::lucario::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        phx::Hash40
    },
    smashline::*,
};

mod mega_evolve;

pub fn install() {
    mega_evolve::install();
}