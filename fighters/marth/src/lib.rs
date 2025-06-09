use {
    exo_utils::{
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        globals::*,
        marth::*,
    },
    smash::{
        app::{
            lua_bind::{
                KineticEnergy,
                *
            },
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
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