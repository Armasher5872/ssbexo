use {
  exo_utils::{
    check_attack::*,
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
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon
  },
  smash_script::macros::*,
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