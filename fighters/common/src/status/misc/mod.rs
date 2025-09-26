use {
    exo_utils::fighter_common::*,
    exo_var::{
        consts::*,
        globals::*,
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
    smash_script::{
        macros::*,
        *
    },
};

mod dead;
mod rebirth;

pub fn install() {
    dead::install();
    rebirth::install();
}