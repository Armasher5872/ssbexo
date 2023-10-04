use {
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            globals::*,
        }
    },
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    }
};

mod attackairlanding;
mod landing;
mod landinglight;

pub fn install() {
    attackairlanding::install();
    landing::install();
    landinglight::install();
}