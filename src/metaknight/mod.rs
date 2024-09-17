use {
    crate::functions::{
        ext::{
            fighter::{
                common::*,
                metaknight::*,
            },
            utility::misc::*,
        },
        var::{
            globals::*,
            metaknight::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "metaknight", "galaxiabeam", false);
    add_param_object("metaknight", "param_glide");
}