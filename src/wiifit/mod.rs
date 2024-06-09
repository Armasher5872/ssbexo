use {
  crate::functions::{
    ext::*,
    var::{
      consts::*,
      globals::*,
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
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}