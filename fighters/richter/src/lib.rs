use {
  exo_utils::{
    extern_func::*,
    fighter_common::*,
    status_end_control::*,
    vector::*,
  },
  exo_var::{
    consts::*,
    globals::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::{
      L2CAgent,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon,
    phx::{
      Vector2f,
      Vector3f
    }
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