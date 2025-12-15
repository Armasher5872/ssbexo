use {
  exo_var::globals::*,
  smash::{
    app::{
      lua_bind::*,
      *
    },
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon
  },
  smashline::*,
};

mod resurrection_book;
mod resurrection_thundersword;

pub fn install() {
    resurrection_book::install();
    resurrection_thundersword::install();
}