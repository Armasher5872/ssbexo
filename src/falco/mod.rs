use {
  crate::functions::var::globals::*,
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::Hash40
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
  update_weapon_count(*WEAPON_KIND_FALCO_BLASTER_BULLET, 1);
}