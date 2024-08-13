use {
    crate::functions::var::rosetta::*,
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
mod grounded;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    grounded::install();
    specials::install();
    throws::install();
}