use {
  crate::{
    functions::{
      ext::*,
      var::{
        consts::*,
        globals::*,
        littlemac::*,
        variables::*,
      },
      util::*,
    },
    littlemac::hook::*,
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
pub mod hook;
mod opff;
mod status;

pub fn install() {
  acmd::install();
  hook::install();
  opff::install();
  status::install();
}