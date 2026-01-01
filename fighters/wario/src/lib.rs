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
    wario::*,
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
    phx::{
      Hash40,
      Vector3f
    }
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