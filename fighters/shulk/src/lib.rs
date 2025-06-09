use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::globals::*,
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::{
      L2CValue,
      lua_const::*,
    }
  }
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}