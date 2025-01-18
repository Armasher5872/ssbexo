#![allow(unused_parens)]
use {
    crate::functions::var::globals::*,
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
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
};

mod catch;
mod catchdash;
mod catchturn;

pub fn install() {
    catch::install();
    catchdash::install();
    catchturn::install();
}