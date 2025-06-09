use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
    ui_manager::*,
  },
  exo_var::{
    globals::*,
    mariod::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
  },
  smash_script::*,
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