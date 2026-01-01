use {
  exo_utils::{
    damage::*,
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::{
    globals::*,
    consts::*,
    variables::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon,
  }
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}