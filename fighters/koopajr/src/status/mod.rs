use {
    exo_utils::fighter_common::*,
    exo_var::{
        globals::*,
        koopajr::*,
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
            L2CWeaponCommon
        },
        phx::Hash40
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::*,
};

mod attack_lw4;
mod cannonball_shoot;
mod float;

pub fn install() {
    attack_lw4::install();
    cannonball_shoot::install();
    float::install();
}