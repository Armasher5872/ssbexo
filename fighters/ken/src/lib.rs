use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        fighter::ken::*,
    },
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod acmd;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
}