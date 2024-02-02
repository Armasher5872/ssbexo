use {
    crate::functions::{
        ext::*,
        var::metaknight::*,
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
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod grounded;
mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;

pub fn install() {
    grounded::install();
    tilts::install();
    smashes::install();
    aerials::install();
    throws::install();
    specials::install();
}