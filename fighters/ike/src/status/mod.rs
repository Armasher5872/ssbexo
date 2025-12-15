use {
    exo_utils::{
        check_attack::*,
        fighter_common::*,
        ike::*,
        weapon::*,
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
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::{
        Hash40,
        Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod slash_shoot;
mod special_lw;
mod special_n_end_max;
mod special_n;

pub fn install() {
    slash_shoot::install();
    special_lw::install();
    special_n_end_max::install();
    special_n::install();
}