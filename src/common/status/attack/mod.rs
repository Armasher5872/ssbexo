#![allow(unused_macros)]
use {
    crate::functions::{
        ext::*,
        var::{
            armstrong::*,
            consts::*,
            globals::*,
            ike::*,
            link::*,
            robot::*,
            variables::*,
        }
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
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
};

mod attack;
pub mod attackair;
mod attackdash;
mod attackhi4;
mod attacklw3;
mod attacklw4;
mod attacks4;

pub fn install() {
    attack::install();
    attackair::install();
    attackdash::install();
    attackhi4::install();
    attacklw3::install();
    attacklw4::install();
    attacks4::install();
}