use {
    crate::functions::{
        ext::fighter::common::*,
        var::globals::*,
    },
    smash::lua2cpp::L2CFighterCommon,
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}