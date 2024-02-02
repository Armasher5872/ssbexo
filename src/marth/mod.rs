use {
    crate::functions::{
        var::{
            consts::*,
            globals::*,
        }
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
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