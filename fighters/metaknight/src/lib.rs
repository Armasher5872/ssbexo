use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            vtable_funcs::*,
        },
        fighter::metaknight::*,
        structs::{
            collision_struct::*,
            shielddata_struct::*,
            vector::*,
        }
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
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "metaknight", "beam", false);
}