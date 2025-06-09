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
        },
        phx::Hash40
    },
    smash_script::*,
    std::arch::asm
};

mod jump;
mod jumpsquat;
mod treadjump;

pub fn install() {
    jump::install();
    jumpsquat::install();
    treadjump::install();
}