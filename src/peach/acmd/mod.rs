use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    specials::install();
    throws::install();
    tilts::install();
}