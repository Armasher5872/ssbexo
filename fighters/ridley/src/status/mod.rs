use {
    exo_var::globals::*,
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
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod attack_lw4;

pub fn install() {
    attack_lw4::install();
}