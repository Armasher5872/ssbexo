use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        fighter::krool::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        krool::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lua2cpp::L2CFighterCommon,
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