use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::{
    gaogaen::*,
    globals::*,
  },
  smash::app::{
    lua_bind::*,
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