use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}