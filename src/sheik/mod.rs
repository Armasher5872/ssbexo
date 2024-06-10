#![allow(unused_assignments)]
use {
  crate::functions::var::{
    consts::*,
    globals::*,
    sheik::*,
    variables::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::{
      Hash40,
      Vector3f
    }
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