use {
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            globals::*,
            metaknight::*,
            variables::*,
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
mod opff;
mod status;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    clone_weapon("koopajr", "cannonball", "metaknight", "galaxiabeam", false);
    add_param_object("metaknight", "param_glide");
}