use {
    exo_utils::{
        collision_struct::*,
        fighter_common::*,
        hook::*,
        shielddata_struct::*,
        status_end_control::*,
        vector::*,
        vtable_funcs::*,
    },
    exo_var::{
        globals::*,
        metaknight::*,
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
        phx::{
            Hash40,
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
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "metaknight", "beam", false);
}