use {
  crate::functions::var::{
    globals::*,
    palutena::*,
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
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}