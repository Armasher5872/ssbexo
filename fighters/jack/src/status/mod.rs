use {
    exo_utils::fighter_common::*,
    exo_var::globals::*,
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
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod dispatch;
mod fire_hit;
mod summon;

pub fn install() {
    dispatch::install();
    fire_hit::install();
    summon::install();
}