#![allow(unused_must_use, unused_assignments)]
use {
    crate::functions::var::{
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
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    }
};

mod attackairlanding;
mod landing;
mod landinglight;

pub fn install() {
    attackairlanding::install();
    landing::install();
    landinglight::install();
}