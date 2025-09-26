use {
    exo_utils::{
        fighter_common::*,
        vector::*,
        status_end_control::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        koopa::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Vector3f
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