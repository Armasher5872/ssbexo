use {
  exo_utils::{
    collision_struct::*,
    damage::*,
    fighter_common::*,
    hook::*,
    status_end_control::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    pikachu::*,
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
    lua2cpp::L2CFighterCommon,
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