use {
    exo_utils::vector::*,
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
        phx::Vector3f
    },
    smash_script::*,
};

mod shouldered_donkey;
mod thrown;

pub fn install() {
    shouldered_donkey::install();
    thrown::install();
}