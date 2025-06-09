use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    ui_manager::*,
  },
  exo_var::{
    globals::*,
    robot::*,
  },
  smash::{
    app::{
      BattleObjectModuleAccessor,
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