use {
  exo_utils::{
    fighter_common::*,
    ui_manager::*,
    vector::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    palutena::*,
  },
  param_config::*,
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::Vector3f
  },
  smash_script::{
    *,
    macros::*
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