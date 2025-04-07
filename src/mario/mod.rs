use {
    crate::functions::{
        ext::fighter::common::*,
        var::{
            consts::*,
            globals::*,
            mario::*,
        }
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
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
    clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "mario", "hammer", false);
}