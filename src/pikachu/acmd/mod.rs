use {
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

mod grounded;
mod smashes;
mod aerials;
mod throws;

pub fn install() {
  grounded::install();
  smashes::install();
  aerials::install();
  throws::install();
}