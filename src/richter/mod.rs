use {
  crate::functions::var::{
    consts::*,
    globals::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon
  },
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}