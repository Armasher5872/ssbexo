use {
    crate::functions::var::ryu::*,
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

mod grounded;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    grounded::install();
    specials::install();
    throws::install();
    tilts::install();
}