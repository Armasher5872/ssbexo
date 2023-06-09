use {
    crate::functions::{
        ext::*,
        var::globals::*,
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
        phx::Hash40
    },
    smash_script::*,
};

pub mod dash;
mod run;

pub fn install() {
  dash::install();
  run::install();
}