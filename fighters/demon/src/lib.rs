use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::demon::*,
        structs::{
            buttons::*,
            collision_struct::*,
            getter_funcs::*,
        }
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
    },
    /*
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    */
    smashline::*,
    smash_script::macros::*,
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