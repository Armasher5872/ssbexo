use {
    crate::functions::{
        ext::utility::boma_ext::*,
        var::lucario::*,
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

mod aerials;
mod grounded;
//mod lucariom;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    //lucariom::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}