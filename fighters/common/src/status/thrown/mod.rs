use {
    exo_utils::fighter::armstrong::*,
    exo_var::consts::*,
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

mod capture;
mod catched_air_ganon;

pub fn install() {
    capture::install();
    catched_air_ganon::install();
}