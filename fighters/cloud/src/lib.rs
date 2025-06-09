use {
  exo_utils::{
    collision_struct::*,
    fighter_common::*,
    shielddata_struct::*,
    status_end_control::*,
    ui_manager::*,
    vector::*,
    vtable_funcs::*,
  },
  exo_var::{
    cloud::*,
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
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon,
    phx::{
      Hash40,
      Vector3f
    }
  }
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
}