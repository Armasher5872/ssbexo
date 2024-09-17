use {
  crate::functions::{
    ext::{
      fighter::common::*,
      utility::{
        boma_ext::*,
        commandcat::*,
      }
    },
    var::{
      dedede::*,
      globals::*,
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
    lua2cpp::L2CFighterCommon
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