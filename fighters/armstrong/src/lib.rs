#![allow(unused_assignments)]
use {
    exo_utils::{
        catch::*,
        fighter_common::*,
        hook::*,
        status_end_control::*,
        vtable_funcs::*,
    },
    exo_var::{
        armstrong::*,
        consts::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    smash_script::*,
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