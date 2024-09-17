use {
  crate::functions::{
    ext::utility::misc::*,
    var::{
      consts::*,
      gekkouga::*,
      globals::*,
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
    phx::{
      Hash40,
      Vector2f,
      Vector3f
    }
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