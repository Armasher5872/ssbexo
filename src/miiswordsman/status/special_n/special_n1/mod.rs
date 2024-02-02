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
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
};

pub mod special_n1;
pub mod special_n1_attack;
pub mod special_n1_loop;