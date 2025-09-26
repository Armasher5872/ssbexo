use {
  exo_utils::{
    cloud::*,
    collision_struct::*,
    fighter_common::*,
    hook::*,
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
  param_config::*,
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
  },
  smash_script::macros::*,
  std::ops::Add
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  status::install();
  vtable::install();
}