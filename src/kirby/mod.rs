use {
  crate::functions::{
    ext::utility::boma_ext::*,
    var::{
      consts::*,
      globals::*,
      kirby::*,
      link::*,
      samusd::*,
      variables::*,
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
mod opff;
mod status;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
}