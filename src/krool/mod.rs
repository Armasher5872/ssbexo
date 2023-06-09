use {
    crate::functions::{
        var::globals::*,
        variables::*,
    },
    smash::{
        app::lua_bind::*,
        hash40,
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