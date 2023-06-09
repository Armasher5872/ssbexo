use {
    crate::functions::{
        ext::*,
        var::globals::*,
    },
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    }
};

mod landing;
mod landinglight;

pub fn install() {
    landing::install();
    landinglight::install();
}