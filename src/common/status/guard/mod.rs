use {
    crate::functions::var::{
        consts::*,
        globals::*,
        sonic::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
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

mod guard;
mod guarddamage;

pub fn install() {
    guard::install();
    guarddamage::install();
}