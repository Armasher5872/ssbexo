use {
  exo_utils::{
    fighter_common::*,
    ganon::*,
    gekkouga::*,
    ike::*,
    metaknight::*,
    pfushigisou::*,
    purin::*,
    status_end_control::*,
    weapon::*,
  },
  exo_var::{
    globals::*,
    link::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
  },
  smashline::*,
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}