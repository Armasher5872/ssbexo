use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::wario::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        wario::*,
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
    smashline::*,
    smash_script::macros::*,
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
    unsafe {
        FIGHTER_WARIO_GENERATE_ARTICLE_KAMIKAZE += clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "wario", "fireballcloned", false);
    }
}