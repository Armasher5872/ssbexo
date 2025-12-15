use {
    exo_utils::{
        extern_func::*,
        fighter_common::*,
        hook::*,
        inkling::*,
        status_end_control::*,
    },
    exo_var::{
        globals::*,
        inkling::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector2f
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
    update_weapon_count(*WEAPON_KIND_INKLING_ROLLERINK, 20);
}