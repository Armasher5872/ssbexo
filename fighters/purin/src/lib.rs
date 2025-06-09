#![allow(unused_parens)]
use {
  exo_utils::{
    fighter_common::*,
    purin::*,
    status_end_control::*,
    weapon::*,
  },
  exo_var::{
    globals::*,
    purin::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::Vector3f
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
  clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "purin", "disarmingvoice", false);
}