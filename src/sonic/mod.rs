use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::boma_ext::*,
        },
        var::{
            consts::*,
            globals::*,
            sonic::*,
        },
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
}