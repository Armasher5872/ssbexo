use {
    crate::functions::var::consts::*,
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
            Vector2f,
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
mod throw;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    smashes::install();
    specials::install();
    throw::install();
    tilts::install();
}