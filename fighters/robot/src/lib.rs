use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    ui_manager::*,
  },
  exo_var::globals::*,
  smash::{
    app::{
      BattleObjectModuleAccessor,
      lua_bind::*,
      *
    },
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon
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