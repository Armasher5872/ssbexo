use {
  exo_utils::{
    collision_struct::*,
    fighter_common::*,
    hook::*,
    status_end_control::*,
    waza_customize::*,
  },
  exo_var::globals::*,
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
    lua2cpp::L2CFighterCommon
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