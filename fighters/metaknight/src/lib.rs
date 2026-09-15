use {
    crate::status::special_lw::*,
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            vtable_funcs::*,
            waza_customize::*,
        },
        fighter::metaknight::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
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
pub mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
    unsafe {
        FIGHTER_METAKNIGHT_GENERATE_ARTICLE_BEAM += clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "metaknight", "cannonballcloned", false);
    }
}