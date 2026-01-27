use {
    exo_utils::{
        check_attack::*,
        fighter_common::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        mario::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::*,
};

mod attack_air;
mod special_lw;
mod special_n_attack;
mod special_n;
mod special_s_bonk;
mod special_s_jump;
mod special_s_landing;
mod special_s_loop;
mod special_s;

pub fn install() {
    attack_air::install();
    special_lw::install();
    special_n_attack::install();
    special_n::install();
    special_s_bonk::install();
    special_s_jump::install();
    special_s_landing::install();
    special_s_loop::install();
    special_s::install();
}