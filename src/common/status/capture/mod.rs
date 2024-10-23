#![allow(unused_must_use)]
use {
    crate::functions::var::{
        chrom::*,
        consts::*,
        littlemac::*,
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
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    }
};

mod capturepulled;
mod set_invalid_capture;

pub fn install() {
    capturepulled::install();
    set_invalid_capture::install();
}