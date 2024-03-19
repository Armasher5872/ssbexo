use {
    crate::functions::var::{
        consts::*,
        globals::*,
        variables::*,
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
        phx::Hash40
    },
    smash_script::*,
};

mod jumpsquat;
mod treadjump;

pub fn install() {
    jumpsquat::install();
    treadjump::install();
}