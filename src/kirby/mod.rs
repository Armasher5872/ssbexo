use {
  crate::functions::{
    ext::utility::boma_ext::*,
    var::{
      globals::*,
      kirby::*,
      link::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::{
      L2CValue,
      lua_const::*,
    },
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