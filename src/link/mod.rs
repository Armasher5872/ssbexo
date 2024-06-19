#![allow(unused_assignments, unused_parens)]
use {
  crate::functions::{
    ext::{
      fighter::link::*,
      status::attack_xx4::*,
    },
    var::{
      consts::*,
      globals::*,
      kirby::*,
      link::*,
      murabito::*,
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
    lua2cpp::{
      L2CFighterCommon,
      *
    },
    phx::{
      Vector2f,
      Vector3f,
      Vector4f
    }
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