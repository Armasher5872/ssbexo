use {
  exo_utils::{
    fighter_common::*,
    hook::*,
    status_end_control::*,
  },
  exo_var::{
    globals::*,
    rockman::*,
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
    phx::Hash40
  },
  smash_script::{
    macros::*,
    *
  },
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
}