use {
  exo_utils::{
    damage::*,
    fighter_common::*,
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
    lib::lua_const::*,
    lua2cpp::*,
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