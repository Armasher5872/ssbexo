use {
    exo_var::{
        consts::*,
        demon::*,
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
        phx::{
            Hash40,
            Vector3f
        }
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