use {
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
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod breath_move;
mod special_lw;
mod special_n;
mod special_s_landing;

pub fn install() {
    breath_move::install();
    special_lw::install();
    special_n::install();
    special_s_landing::install();
}