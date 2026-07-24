use {
    exo_var::globals::*,
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::*
    },
    smashline::*,
};

mod guard_off;
mod wait;

pub fn install() {
    guard_off::install();
    wait::install();
}