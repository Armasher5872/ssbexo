use {
    crate::functions::var::{
        consts::*,
        kirby::*,
    },
    smash::{
        app::{
            lua_bind::{
                FighterCutInManager,
                *
            },
            sv_animcmd::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod grounded;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}