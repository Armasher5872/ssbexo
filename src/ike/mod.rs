use {
  crate::functions::{
    ext::*,
    var::{
      consts::*,
      globals::*,
      ike::*,
      variables::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
mod opff;
mod status;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
  clone_weapon("koopajr", "cannonball", "ike", "slash", false);
}