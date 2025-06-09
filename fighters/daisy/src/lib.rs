use {
  exo_utils::{
    daisy::*,
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::globals::*,
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon
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
  update_weapon_count(*WEAPON_KIND_DAISY_KASSAR, 2);
}