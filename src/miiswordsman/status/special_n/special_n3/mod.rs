use {
    crate::functions::{
        ext::*,
        var::{
            globals::*,
            miiswordsman::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
};

pub mod special_n3;
pub mod special_n3_slash;