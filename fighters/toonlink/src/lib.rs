
use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::globals::*,
    smash::{
        app::*,
        lib::lua_const::*,
    },
    smashline::*,
};

mod acmd;
mod opff;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    vtable::install();
}