use {
  exo_utils::{
    collision_struct::*,
    fighter_common::*,
    status_end_control::*,
    vector::*,
  },
  exo_var::{
    globals::*,
    littlemac::*,
    variables::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::Vector3f
  },
  smash_script::*,
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