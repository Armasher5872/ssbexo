use {
  exo_utils::{
    fighter_common::*,
    link::*,
    status_end_control::*,
  },
  exo_var::{
    dedede::*,
    globals::*,
    kirby::*,
    link::*,
    murabito::*,
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
    phx::Vector3f
  },
  smashline::*,
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}