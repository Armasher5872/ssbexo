#![allow(unused_assignments)]
use {
    exo_var::{
        consts::*,
        globals::*,
        palutena::*,
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

mod attackairlanding;
mod landing;
mod landinglight;

pub fn install() {
    attackairlanding::install();
    landing::install();
    landinglight::install();
}