use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    ui_manager::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    link::*,
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