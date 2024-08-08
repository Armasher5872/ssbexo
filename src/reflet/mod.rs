use {
  crate::functions::{
    ext::fighter::common::*,
    var::{
      consts::*,
      globals::*,
    }
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
    lua2cpp::L2CFighterCommon,
    phx::Vector2f
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