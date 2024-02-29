use {
  crate::functions::{
    ext::*,
    var::{
      consts::*,
      globals::*,
      kirby::*,
      link::*,
      murabito::*,
      variables::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::{
      L2CFighterCommon,
      *
    },
    phx::{
      Vector2f,
      Vector3f,
      Vector4f
    }
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
  clone_weapon("koopajr", "cannonball", "link", "clawshot", false);
  clone_weapon("koopajr", "cannonball", "link", "clawshot_hand", false);
}