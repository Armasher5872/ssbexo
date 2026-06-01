use {
    exo_var::{
        captain::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::*,
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    },
};

mod special_lw_bounce;
mod special_lw;
mod special_n_charged;
mod special_n;

pub fn install() {
    special_lw_bounce::install();
    special_lw::install();
    special_n_charged::install();
    special_n::install();
}