use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::globals::*,
  smash::{
    app::*,
    lib::lua_const::*,
  },
  smashline::*,
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
  clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "lucario", "bone", false);
}