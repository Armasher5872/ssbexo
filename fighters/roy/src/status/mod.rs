use {
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smashline::*,
};

mod special_n_loop;
mod special_n_turn;
mod special_s;

pub fn install() {
    special_n_loop::install();
    special_n_turn::install();
    special_s::install();
}