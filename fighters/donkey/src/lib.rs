use {
    exo_utils::{
        collision_struct::*,
        damage::*,
        donkey::*,
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        consts::*,
        donkey::*,
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
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "donkey", "barrel", false);
    clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "donkey", "barrel_cannon", false);
}