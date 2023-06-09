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
mod throws;
mod specials;

pub fn install() {
  grounded::install();
  smashes::install();
  throws::install();
  specials::install();
}