use {
    exo_utils::armstrong::*,
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
        phx::*,
    }
};

mod catched_air_ganon;
mod shouldered_donkey;

pub fn install() {
    catched_air_ganon::install();
    shouldered_donkey::install();
}