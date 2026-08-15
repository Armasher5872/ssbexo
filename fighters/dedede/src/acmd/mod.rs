use {
    exo_var::consts::*,
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
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod aerials;
mod grounded;
mod other;
mod smashes;
mod throws;

pub fn install() {
    aerials::install();
    grounded::install();
    other::install();
    smashes::install();
    throws::install();
}