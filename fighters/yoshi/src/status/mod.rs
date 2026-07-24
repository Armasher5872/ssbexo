use {
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smashline::*,
    smash_script::*,
};

mod guard_off;
mod guard_on;

pub fn install() {
    guard_off::install();
    guard_on::install();
}