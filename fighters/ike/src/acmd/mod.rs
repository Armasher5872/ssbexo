use {
    exo_utils::fighter_common::*,
    exo_var::{
        consts::*,
        ike::*,
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
mod smashes;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    grounded::install();
    smashes::install();
    specials::install();
    throws::install();
}