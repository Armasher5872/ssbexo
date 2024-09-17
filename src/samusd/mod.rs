#![allow(unused_parens)]
use {
    crate::functions::var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
}