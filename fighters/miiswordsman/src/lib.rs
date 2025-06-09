use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    waza_customize::*,
  },
  exo_var::{
    globals::*,
    miiswordsman::*,
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
  },
  smash_script::*,
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}