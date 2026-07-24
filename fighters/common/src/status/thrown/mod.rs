use {
    exo_utils::fighter::armstrong::*,
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
        phx::*,
    },
    smash_script::*,
};

mod capture;
mod catched_air_ganon;
mod shouldered_donkey;

pub fn install() {
    capture::install();
    catched_air_ganon::install();
    shouldered_donkey::install();
}