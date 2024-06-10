#![allow(dead_code, improper_ctypes_definitions)]
use {
  crate::{
    functions::{
      ext::utility::{
        boma_ext::*,
        get_objects::*,
        misc::*,
      },
      var::{
        consts::*,
        globals::*,
        littlemac::*,
        variables::*,
      }
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
mod hook;
mod opff;
mod status;

pub fn install() {
  acmd::install();
  hook::install();
  opff::install();
  status::install();
}