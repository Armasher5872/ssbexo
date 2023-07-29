use {
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            donkey::*,
        }
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
  acmd::install();
  opff::install();
}