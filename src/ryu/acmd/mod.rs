use {
    crate::functions::var::{
        consts::*,
        ryu::*,
    },
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
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    grounded::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}