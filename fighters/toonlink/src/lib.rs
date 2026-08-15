
use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::globals::*,
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}