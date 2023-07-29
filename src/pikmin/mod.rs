use {
  smash::{
    app::lua_bind::*,
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
  },
  smashline::*,
};

mod opff;

pub fn install() {
  opff::install();
}