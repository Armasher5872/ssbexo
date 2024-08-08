#![allow(unused_must_use)]
use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            commandcat::*,
        },
        var::{
            globals::*,
            variables::*,
        }
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
};

mod specialhi;
mod sub_transition_check_special;

pub fn install() {
    specialhi::install();
    sub_transition_check_special::install();
}