#![allow(unused_parens)]
use {
    crate::functions::{
        ext::fighter::murabito_shizue_common::*,
        var::{
            consts::*,
            murabito::*,
            variables::*,
        }
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::*,
    },
    smashline::*,
};

mod opff;
mod status;

pub fn install() {
    opff::install();
    status::install();
}