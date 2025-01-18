#![allow(unused_assignments)]
use {
  crate::functions::{
    ext::fighter::common::*,
    var::{
      consts::*,
      globals::*,
      sheik::*,
    }
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
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}