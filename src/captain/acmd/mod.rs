use {
    crate::functions::var::{
        captain::*,
        kirby::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f
        }
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod smashes;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    smashes::install();
    specials::install();
    throws::install();
}