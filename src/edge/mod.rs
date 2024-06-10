use {
  crate::functions::{
    ext::utility::{
      boma_ext::*,
      misc::*,
    },
    var::{
      consts::*,
      edge::*,
      globals::*,
      variables::*,
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
    phx::Vector3f
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