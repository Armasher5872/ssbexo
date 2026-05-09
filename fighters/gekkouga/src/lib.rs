use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        fighter::gekkouga::*,
    },
    exo_var::globals::*,
    smash::{
        app::*,
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
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "gekkouga", "mat", false);
}