use {
    crate::functions::ext::fighter::inkling::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    throws::install();
    tilts::install();
}