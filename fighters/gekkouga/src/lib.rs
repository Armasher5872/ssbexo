use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::{
    gekkouga::*,
    globals::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
  },
  smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
  clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "gekkouga", "mat", false);
}