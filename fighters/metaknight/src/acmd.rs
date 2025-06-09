use {
    exo_utils::fighter_common::*,
    exo_var::{
        consts::*,
        metaknight::*,
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
        hash40,
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