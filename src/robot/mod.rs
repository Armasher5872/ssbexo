use {
  crate::functions::var::{
    consts::*,
    globals::*,
    robot::*,
    variables::*,
  },
  smash::{
    app::{
      lua_bind::*,
      sv_information
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
mod opff;
mod status;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
}