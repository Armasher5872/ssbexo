use {
  crate::functions::var::{
    consts::*,
    globals::*,
    variables::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon
  },
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