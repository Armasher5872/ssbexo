use {
  exo_utils::{
    command_input::*,
    fighter_common::*,
    hook::*,
    status_end_control::*,
    ui_manager::*,
  },
  exo_var::{
    consts::*,
    globals::*,
    lucario::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    phx::Hash40
  },
  smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
  vtable::install();
  clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "lucario", "bone", false);
}