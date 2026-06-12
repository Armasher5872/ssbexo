use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::donkey::*,
        status::damage::*,
        structs::collision_struct::*,
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
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "donkey", "barrel", false);
    clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "donkey", "barrel_cannon", false);
}