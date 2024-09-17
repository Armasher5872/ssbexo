use {
  crate::functions::{
    ext::utility::{
      boma_ext::*,
      commandcat::*,
      misc::*,
    },
    var::{
      consts::*,
      globals::*,
      miifighter::*,
    }
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon,
    phx::Vector3f
  },
  smash_script::*,
  smashline::*,
};

mod acmd;
pub mod status;

pub fn install() {
  acmd::install();
  status::install();
}