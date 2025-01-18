#![allow(unused_assignments, unused_parens)]
use {
    crate::functions::{
        ext::{
            status::damage::*,
            utility::misc::*,
        },
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
mod damagefall;
mod damagefly;
mod damageflyroll;
mod damagelanding;
//mod damagesleep;
mod damagesleepfall;
//mod damagesong;
mod furafura;
mod saving;
mod shieldbreakfly;
mod treadfall;

pub fn install() {
    damage::install();
    damageair::install();
    damagefall::install();
    damagefly::install();
    damageflyroll::install();
    damagelanding::install();
    //damagesleep::install();
    damagesleepfall::install();
    //damagesong::install();
    furafura::install();
    saving::install();
    shieldbreakfly::install();
    treadfall::install();
}