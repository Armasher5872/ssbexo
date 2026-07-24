use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::{
        globals::*,
        lucario::*,
    },
    smash::{
        app::*,
        lib::lua_const::*,
    },
    smashline::*,
};

mod acmd;
mod opff;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    vtable::install();
    unsafe {
        FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE += clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "lucario", "swordcloned", false);
    }
}