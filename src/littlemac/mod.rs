use {
  crate::functions::{
    ext::*,
    var::variables::*,
  },
  smash::{
    app::lua_bind::*,
    lib::lua_const::*,
    lua2cpp::*,
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
}