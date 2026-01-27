use {
    exo_utils::vector::*,
    exo_var::{
        consts::*,
        globals::*,
        sonic::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod special_lw_hold;
mod special_lw;
mod special_n_cancel;
mod special_n_fail;
mod special_n_hit;
mod special_n_rebound;
mod special_n;
mod special_s_rush;
mod special_s_rush_end;
mod special_s;

pub fn install() {
    special_lw_hold::install();
    special_lw::install();
    special_n_cancel::install();
    special_n_fail::install();
    special_n_hit::install();
    special_n_rebound::install();
    special_n::install();
    special_s_rush::install();
    special_s_rush_end::install();
    special_s::install();
}