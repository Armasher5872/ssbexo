use {
    exo_utils::fighter_common::*,
    exo_var::globals::*,
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

mod rebirth;

pub fn install() {
    rebirth::install();
}