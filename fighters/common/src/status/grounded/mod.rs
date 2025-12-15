use {
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
};

mod appeal;
mod run;
mod turndash;
mod wait;

pub fn install() {
    appeal::install();
    run::install();
    turndash::install();
    wait::install();
}