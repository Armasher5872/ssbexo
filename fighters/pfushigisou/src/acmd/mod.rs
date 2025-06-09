use {
    exo_var::{
        consts::*,
        pfushigisou::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod aerials;
mod grounded;
mod specials;
mod smashes;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    specials::install();
    smashes::install();
    throws::install();
    tilts::install();
}