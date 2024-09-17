use {
  crate::functions::var::globals::*,
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
mod status;

pub fn install() {
  acmd::install();
  status::install();
}