use {
  crate::functions::{
    ext::{
      fighter::{
        common::*,
        ike::*,
      },
      utility::misc::*,
    },
    var::{
      consts::*,
      globals::*,
      ike::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::Vector2f
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
  clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "ike", "slash", false);
}