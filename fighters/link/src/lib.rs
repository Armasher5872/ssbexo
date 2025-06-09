use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    link::*,
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
    phx::Hash40
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