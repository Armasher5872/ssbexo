use {
    exo_var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
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

mod grounded;
mod smashes;
mod throws;
mod tilts;

pub fn install() {
    grounded::install();
    smashes::install();
    throws::install();
    tilts::install();
}