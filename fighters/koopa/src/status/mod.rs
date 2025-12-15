use {
    exo_utils::{
        fighter_common::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        koopa::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Vector3f
    },
    smash_script::*,
    smashline::*,
};

mod firebreath_move;
mod special_hi_a;
mod special_lw_a;
mod special_n;

pub fn install() {
    firebreath_move::install();
    special_hi_a::install();
    special_lw_a::install();
    special_n::install();
}