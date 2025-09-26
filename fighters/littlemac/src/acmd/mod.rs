use {
    exo_var::{
        consts::*,
        littlemac::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                get_value_float,
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