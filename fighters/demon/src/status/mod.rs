use {
    exo_var::{
        demon::*,
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
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack;
mod flash_punch;
mod landing_attack_air;
mod special_lw;

pub fn install() {
    attack::install();
    flash_punch::install();
    landing_attack_air::install();
    special_lw::install();
}