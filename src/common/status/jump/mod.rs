use {
    crate::functions::var::globals::*,
    smash::{
        app::lua_bind::*,
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

pub fn install() {
    jumpsquat::install();
}