#![allow(unused_must_use)]
use {
    crate::functions::{
        ext::{
            fighter::donkey::*,
            utility::{
                boma_ext::*,
                misc::*,
            }
        },
        var::{
            consts::*,
            donkey::*,
            globals::*,
            variables::*,
        }
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
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod hook;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    hook::install();
    opff::install();
    status::install();
}