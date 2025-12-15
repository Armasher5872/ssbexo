use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::{
    edge::*,
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
  },
  smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
  vtable::install();
}