#![allow(unused_must_use)]
use {
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
};

mod sub_transition_check_special;

pub fn install() {
    sub_transition_check_special::install();
}