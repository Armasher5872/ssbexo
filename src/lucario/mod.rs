use {
  crate::functions::var::lucario::*,
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

pub fn install() {
  acmd::install();
  opff::install();
  status::install();
  clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "lucario", "bone", false);
}