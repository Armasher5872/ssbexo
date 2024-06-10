#![allow(unused_parens)]
use {
  crate::functions::var::{
    consts::*,
    globals::*,
    link::*,
    rosetta::*,
    variables::*,
  },
  smash::{
    app::lua_bind::*,
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
  },
  smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
  acmd::install();
  opff::install();
}