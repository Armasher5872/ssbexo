use {
    exo_utils::common::check_attack::*,
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    },
    smash_script::macros::*,
    smashline::*,
};

mod appeal;
mod attack_air;
mod attack_s4;

pub fn install() {
    appeal::install();
    attack_air::install();
    attack_s4::install();
}