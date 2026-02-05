use {
    exo_utils::{
        collision_struct::*,
        damage::*,
        fighter_common::*,
        hook::*,
        sonic::*,
        ui_manager::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        sonic::*,
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
        phx::*
    },
    smash_script::{
        macros::*,
        *
    }
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
}