use {
    exo_utils::{
        fighter::armstrong::*,
        status::catch::*,
        structs::getter_funcs::*,
    },
    exo_var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                execute,
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

mod aerials;
mod grounded;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}