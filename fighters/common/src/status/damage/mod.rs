#![allow(unused_assignments)]
use {
    exo_utils::{
        damage::*,
        knockback::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
    },
    skyline::hooks::InlineCtx,
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
    smash_script::{
        *,
        macros::*
    },
};

mod damage;
mod damageair;
mod damagefall;
mod damagefly;
mod damageflyroll;
//mod damagesleep;
mod damagesleepfall;
//mod damagesong;
mod furafura;
mod saving;

pub fn install() {
    damage::install();
    damageair::install();
    damagefall::install();
    damagefly::install();
    damageflyroll::install();
    //damagesleep::install();
    damagesleepfall::install();
    //damagesong::install();
    furafura::install();
    saving::install();
}