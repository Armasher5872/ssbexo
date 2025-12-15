use {
    exo_utils::catch::*,
    exo_var::{
        armstrong::*,
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
        phx::Hash40,
    },
    smash_script::*,
    smashline::*,
};

mod attack_air;
mod final_smash;
mod special_air_s_catch;
mod special_n;
mod special_s;
mod throw;

pub fn install() {
    attack_air::install();
    final_smash::install();
    special_air_s_catch::install();
    special_n::install();
    special_s::install();
    throw::install();
}