#![allow(unused_parens)]
use {
  crate::functions::{
    ext::{
      fighter::{
        common::*,
        purin::*,
      },
      utility::{
        boma_ext::*,
        misc::*,
      }
    },
    var::{
      globals::*,
      purin::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
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
  clone_weapon("ganon", "sword", "purin", "microphone", false);
  clone_weapon("koopajr", "cannonball", "purin", "disarmingvoice", false);
}