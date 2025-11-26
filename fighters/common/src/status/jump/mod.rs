use {
    exo_utils::buttons::*,
    exo_var::{
        consts::*,
        globals::*,
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
        }
    }
};

mod jump;
mod jumpsquat;

pub fn install() {
    jump::install();
    jumpsquat::install();
}