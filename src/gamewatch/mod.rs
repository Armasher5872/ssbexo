use {
    crate::functions::{
        ext::utility::misc::*,
        var::{
            globals::*,
            variables::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValueType,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
  acmd::install();
  opff::install();
}