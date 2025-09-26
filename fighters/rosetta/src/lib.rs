use {
  exo_utils::{
    donkey::*,
    fighter_common::*,
    ganon::*,
    gekkouga::*,
    ike::*,
    metaknight::*,
    pfushigisou::*,
    purin::*,
    status_end_control::*,
    vector::*,
    weapon::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    link::*,
    rosetta::*,
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
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
}