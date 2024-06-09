use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod escape;
mod grounded;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    escape::install();
    grounded::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}