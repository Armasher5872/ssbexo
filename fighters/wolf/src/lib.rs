use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        status::damage::*,
    },
    exo_var::{
        globals::*,
        consts::*,
    },
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}