use {
    crate::functions::var::robot::*,
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
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}