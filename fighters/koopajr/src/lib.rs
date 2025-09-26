use {
  exo_utils::{
    collision_struct::*,
    fighter_common::*,
    ganon::*,
    hook::*,
    shielddata_struct::*,
    vector::*,
    vtable_funcs::*,
  },
  exo_var::{
    consts::*,
    donkey::*,
    ganon::*,
    gekkouga::*,
    globals::*,
    ike::*,
    koopajr::*,
    variables::*,
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
    phx::{
      Hash40,
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
  status::install();
  vtable::install();
}