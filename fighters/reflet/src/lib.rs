use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    vector::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    reflet::*,
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
    lua2cpp::L2CFighterCommon,
    phx::{
      Vector2f,
      Vector3f
    }
  },
  smash_script::{
    macros::*,
    *
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