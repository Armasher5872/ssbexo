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
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        *,
        Priority::Low
    },
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