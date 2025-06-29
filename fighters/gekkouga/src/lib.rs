use {
  exo_utils::{
    check_attack::*,
    extern_func::*,
    fighter_common::*,
    gekkouga::*,
    weapon::*,
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
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::{
      Hash40,
      Vector2f,
      Vector3f
    }
  },
  smash_script::{
    *,
    macros::*
  },
  smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "gekkouga", "mat", false);
  status::install();
  vtable::install();
}