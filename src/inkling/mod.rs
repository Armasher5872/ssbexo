use {
    crate::functions::{
        ext::utility::misc::*,
        var::globals::*,
        util::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            L2CWeaponCommon
        },
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
}