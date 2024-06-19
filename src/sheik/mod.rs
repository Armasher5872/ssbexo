#![allow(unused_assignments)]
use {
  crate::functions::var::{
    globals::*,
    sheik::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
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