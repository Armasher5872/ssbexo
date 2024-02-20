use {
  crate::functions::var::{
    globals::*,
    link::*,
    rosetta::*,
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