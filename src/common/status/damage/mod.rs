use {
    crate::functions::var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::lua_bind::*,
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

mod damagelanding;
mod furafura;
mod saving;
mod shieldbreakfly;

pub fn install() {
    damagelanding::install();
    furafura::install();
    saving::install();
    shieldbreakfly::install();
}