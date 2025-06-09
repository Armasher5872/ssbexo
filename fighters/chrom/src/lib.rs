use {
  exo_utils::{
    check_attack::*,
    fighter_common::*,
    status_end_control::*,
    vector::*,
  },
  exo_var::{
    chrom::*,
    consts::*,
    globals::*,
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