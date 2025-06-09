use {
  exo_utils::{
    extern_func::*,
    fighter_common::*,
    hook::*,
    link::*,
    status_end_control::*,
  },
  exo_var::{
    dedede::*,
    globals::*,
    kirby::*,
    link::*,
    murabito::*,
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
    lua2cpp::L2CFighterCommon,
    phx::Vector3f
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