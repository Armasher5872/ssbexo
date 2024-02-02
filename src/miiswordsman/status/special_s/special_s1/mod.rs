use {
    crate::functions::var::{
        globals::*,
        miiswordsman::*,
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
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
};

pub mod special_s1;
pub mod special_s1_start;
pub mod special_s1_end;