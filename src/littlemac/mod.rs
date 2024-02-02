use {
  crate::{
    functions::{
      ext::*,
      var::{
        consts::*,
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
    lib::lua_const::*,
    lua2cpp::*,
    phx::Vector3f
  },
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