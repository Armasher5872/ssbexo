use {
    common::status::grounded::dash::fgc_dashback_main,
    exo_utils::{
        attack::*,
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        demon::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
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
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
}