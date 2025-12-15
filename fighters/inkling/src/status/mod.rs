use {
    exo_utils::{
        extern_func::*,
        fighter_common::*,
        inkling::*,
        vector::*,
    },
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            L2CWeaponCommon
        },
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod inkbullet_hit;
mod special_s;
mod splashbomb_explode;

pub fn install() {
    inkbullet_hit::install();
    special_s::install();
    splashbomb_explode::install();
}