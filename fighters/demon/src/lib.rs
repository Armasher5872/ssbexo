use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::demon::*,
        structs::collision_struct::*,
    },
    exo_var::{
        demon::*,
        globals::*,
        variables::*,
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
        lua2cpp::*,
        phx::*,
    },
    /*
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    */
    smashline::*
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