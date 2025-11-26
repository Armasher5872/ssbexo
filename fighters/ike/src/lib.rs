use {
  exo_utils::{
    check_attack::*,
    fighter_common::*,
    ike::*,
    status_end_control::*,
    weapon::*,
  },
  exo_var::{
    globals::*,
    ike::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::{
      Hash40,
      Vector3f
    }
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
  clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "ike", "slash", false);
}