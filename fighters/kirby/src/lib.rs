use {
  exo_utils::{
    damage::*,
    fighter_common::*,
    kirby::*,
    status_end_control::*,
  },
  exo_var::{
    consts::*,
    globals::*,
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
    lua2cpp::*,
  }
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
}