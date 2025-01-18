use {
  crate::functions::var::{
    consts::*,
    globals::*,
    robot::*,
  },
  smash::{
    app::{
      BattleObjectModuleAccessor,
      lua_bind::*,
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

pub fn install() {
  acmd::install();
  status::install();
}