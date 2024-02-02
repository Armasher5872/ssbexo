use {
    crate::functions::var::globals::*,
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

pub mod special_hi2;