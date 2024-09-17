use {
  crate::functions::{
    ext::utility::misc::*,
    var::{
      consts::*,
      globals::*,
      rosetta::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
    lua2cpp::*,
    phx::Vector3f
  },
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}