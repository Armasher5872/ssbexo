use {
    crate::common::status::grounded::dash::fgc_dashback_main,
    smash::{
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
  acmd::install();
  status::install();
}