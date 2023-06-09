use {
    crate::common::status::attack::attackair::*,
    smash::{
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}