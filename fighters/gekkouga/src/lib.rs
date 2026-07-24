use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::gekkouga::*,
        structs::collision_struct::*,
    },
    exo_var::{
        gekkouga::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
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
    unsafe {
        FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT += clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "gekkouga", "cannonballcloned", false);
    }
}