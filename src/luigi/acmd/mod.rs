use {
    crate::functions::var::luigi::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            sv_module_access::*,
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