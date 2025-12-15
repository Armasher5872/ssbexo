use {
    exo_utils::{
        check_attack::*,
        fighter_common::*,
        gekkouga::*,
        weapon::*,
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
        lua2cpp::*,
        phx::{
        Hash40,
        Vector2f,
        Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod attack_air;
mod mat_fall;
mod special_lw;
mod special_s_attack;
mod special_s_end;
mod special_s;

pub fn install() {
    attack_air::install();
    mat_fall::install();
    special_lw::install();
    special_s_attack::install();
    special_s_end::install();
    special_s::install();
}