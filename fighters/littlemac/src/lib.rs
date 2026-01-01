use {
  exo_utils::{
    collision_struct::*,
    damage::*,
    fighter_common::*,
    hook::*,
    littlemac::*,
    status_end_control::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    littlemac::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
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