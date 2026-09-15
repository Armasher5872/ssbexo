use {
    exo_utils::fighter::snake::*,
    exo_var::{
        globals::*,
        snake::*,
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    },
    smashline::*,
};

mod attack_s4;
mod special_hi_hang;
mod special_hi;
mod special_n_hold_air;
mod special_n_hold_dash_b;
mod special_n_hold_dash_f;
mod special_n_hold_jump_aerial;
mod special_n_hold_jump;
mod special_n_hold_jumpsquat;
mod special_n_hold_landing;
mod special_n_hold_wait;
mod special_n_hold_walk_b;
mod special_n_hold_walk_brake_b;
mod special_n_hold_walk_brake_f;
mod special_n_hold_walk_f;

pub fn install() {
    attack_s4::install();
    special_hi_hang::install();
    special_hi::install();
    special_n_hold_air::install();
    special_n_hold_dash_b::install();
    special_n_hold_dash_f::install();
    special_n_hold_jump_aerial::install();
    special_n_hold_jump::install();
    special_n_hold_jumpsquat::install();
    special_n_hold_landing::install();
    special_n_hold_wait::install();
    special_n_hold_walk_b::install();
    special_n_hold_walk_brake_b::install();
    special_n_hold_walk_brake_f::install();
    special_n_hold_walk_f::install();
}