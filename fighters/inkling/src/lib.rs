use {
    exo_utils::{
        extern_func::*,
        fighter_common::*,
        hook::*,
        inkling::*,
        status_end_control::*,
        vector::*,
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
        lua2cpp::{
            L2CFighterCommon,
            L2CWeaponCommon
        },
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
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
    update_weapon_count(*WEAPON_KIND_INKLING_ROLLERINK, 20);
}