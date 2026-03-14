use {
  exo_utils::{
    collision_struct::*,
    damage::*,
    fighter_common::*,
    hook::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    luigi::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
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