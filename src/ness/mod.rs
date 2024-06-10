use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::boma_ext::*,
        },
        var::{
            consts::*,
            globals::*,
            variables::*,
        }
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector4f
        }
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}