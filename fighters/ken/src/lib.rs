use {
    common::status::grounded::dash::fgc_dashback_main,
    exo_utils::{
        command_input::*,
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        ken::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    },
    smash_script::{
        *,
        macros::*
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