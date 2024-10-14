use {
  crate::functions::{
    ext::fighter::common::*,
    var::{
      consts::*,
      globals::*,
      roy::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
    lua2cpp::*,
    phx::Hash40
  },
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}