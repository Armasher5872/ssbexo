#![allow(unused_macros)]

use {
    crate::functions::var::{
        consts::*,
        globals::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod escapeair;
mod escapefb;

pub fn install() {
    escapeair::install();
    escapefb::install();
}