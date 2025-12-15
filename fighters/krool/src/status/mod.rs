use {
    exo_utils::fighter_common::*,
    exo_var::{
        consts::*,
        globals::*,
        krool::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod attack_lw4;
mod special_hi_start;
mod special_lw_charge;
mod special_lw_launch;
mod special_lw;

pub fn install() {
    attack_lw4::install();
    special_hi_start::install();
    special_lw_charge::install();
    special_lw_launch::install();
    special_lw::install();
}