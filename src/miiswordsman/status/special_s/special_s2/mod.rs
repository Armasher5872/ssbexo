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

pub mod special_s2;
pub mod special_s2_attack_1;
pub mod special_s2_attack_2;
pub mod special_s2_attack_3;