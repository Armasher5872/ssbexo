#![allow(
    unused_macros,
    unused_variables
)]

use {
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            globals::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod guard;
mod guardoff;

pub fn install() {
  guard::install();
  guardoff::install();
}