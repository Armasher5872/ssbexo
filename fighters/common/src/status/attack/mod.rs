#![allow(unused_comparisons, unused_macros, unused_parens)]
use {
    exo_utils::{
        attack_dash::*,
        attack_xx4::*,
    },
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
            *
        },
        hash40,
        lib::{
            lua_const::*,
            *
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::{
        macros::*,
        *
    }
};

mod attack;
pub mod attackair;
mod attackdash;
mod attackhi4;
mod attacklw4;
mod attacks4;
mod attackxx4;

pub fn install() {
    attack::install();
    attackair::install();
    attackdash::install();
    attackhi4::install();
    attacklw4::install();
    attacks4::install();
    attackxx4::install();
}