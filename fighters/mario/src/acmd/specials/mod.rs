use {
    exo_utils::{
        fighter_common::*,
        vector::*,
    },
    exo_var::mario::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod final_smash;
mod finale_fireball_regular;
mod special_lw;
mod special_n_attack;
mod special_s_bonk;
mod special_s_jump;
mod special_s_landing;
mod special_s_loop;
mod special_s;

pub fn install() {
    final_smash::install();
    finale_fireball_regular::install();
    special_lw::install();
    special_n_attack::install();
    special_s_bonk::install();
    special_s_jump::install();
    special_s_landing::install();
    special_s_loop::install();
    special_s::install();
}