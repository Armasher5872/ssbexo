use {
    exo_utils::{
        collision_struct::*,
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        lucina::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::macros::*,
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