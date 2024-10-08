#![allow(unused_parens)]
use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            commandcat::*,
        },
        var::{
            consts::*,
            globals::*,
            mewtwo::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
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