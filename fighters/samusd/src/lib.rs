use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        status::damage::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        samusd::*,
    },
    smash::{
        app::lua_bind::*,
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

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    update_weapon_count(*WEAPON_KIND_SAMUSD_CSHOT, 1);
}