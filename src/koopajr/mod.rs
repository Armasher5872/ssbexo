#![allow(unused_parens)]
use {
  crate::functions::{
    ext::utility::misc::*,
    var::{
      globals::*,
      koopajr::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::{
      L2CFighterCommon,
      L2CWeaponCommon
    },
    phx::Hash40
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}