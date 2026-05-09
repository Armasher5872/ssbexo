use {
    exo_utils::{
        common::{
            command_input::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::ken::*,
        structs::{
            buttons::cat4::*,
            getter_funcs::*,
        }
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
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}