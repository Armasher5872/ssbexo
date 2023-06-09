use {
  smash::{
    app::lua_bind::*,
    lib::lua_const::*,
    lua2cpp::L2CFighterCommon
  },
  smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
  acmd::install();
  opff::install();
}