use {
  crate::functions::{
    ext::*,
    var::{
      globals::*,
      luigi::*,
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
    phx::{
      Hash40,
      Vector2f
    }
  },
  smashline::*,
};

mod acmd;
//mod opff;
mod status;

pub fn install() {
  acmd::install();
  //opff::install();
  status::install();
}