use {
  crate::functions::{
    ext::utility::misc::*,
    var::globals::*,
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
    lua2cpp::L2CFighterCommon
  },
  smashline::*,
};

mod acmd;
mod opff;
pub mod status;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
}