use {
    crate::functions::var::variables::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod aerials;
mod smashes;
mod throws;

pub fn install() {
  aerials::install();
  smashes::install();
  throws::install();
}