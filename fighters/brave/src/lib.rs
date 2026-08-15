use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::globals::*,
    smash::{
        app::Fighter,
        lua2cpp::*,
    },
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}