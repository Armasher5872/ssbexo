use {
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
        lua2cpp::L2CFighterCommon
    }
};

mod landing_attack_air;
mod landing_fall_special;
mod landing;
mod landinglight;

pub fn install() {
    landing_attack_air::install();
    landing_fall_special::install();
    landing::install();
    landinglight::install();
}