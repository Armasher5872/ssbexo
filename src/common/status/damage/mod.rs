#![allow(unused_must_use, unused_assignments, unused_parens)]
use {
    crate::functions::{
        ext::status::damage::*,
        var::{
            consts::*,
            globals::*,
            littlemac::*,
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
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
};

mod damage;
mod damageair;
mod damagefly;
mod damagelanding;
//mod damagesleep;
mod damagesleepfall;
//mod damagesong;
mod furafura;
mod saving;
mod shieldbreakfly;

pub fn install() {
    damage::install();
    damageair::install();
    damagefly::install();
    damagelanding::install();
    //damagesleep::install();
    damagesleepfall::install();
    //damagesong::install();
    furafura::install();
    saving::install();
    shieldbreakfly::install();
}