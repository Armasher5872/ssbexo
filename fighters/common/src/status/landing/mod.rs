#![allow(unused_assignments)]
use {
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    }
};

mod landing;
mod landinglight;

pub fn install() {
    landing::install();
    landinglight::install();
}