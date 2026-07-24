use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::{
        globals::*,
        ike::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
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
        FIGHTER_IKE_GENERATE_ARTICLE_SLASH += clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "ike", "cannonballcloned", false);
    }
}