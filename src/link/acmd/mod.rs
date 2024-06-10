use {
    crate::functions::{
        ext::fighter::link::*,
        var::link::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f,
            Vector3f,
            Vector4f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod grounded;
mod misc;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    misc::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}