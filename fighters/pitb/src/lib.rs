use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::globals::*,
    smash::{
        lib::L2CValue,
        lua2cpp::*,
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}