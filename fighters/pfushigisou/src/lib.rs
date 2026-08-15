use {
    exo_utils::common::{
        status_end_control::*,
        var_reset::*,
    },
    exo_var::{
        globals::*,
        pfushigisou::*,
    },
    smash::{
        app::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
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
        FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SLUDGE += clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "pfushigisou", "cannonballcloned", false);
    }
}