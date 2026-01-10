use {
    exo_utils::fighter_common::*,
    exo_var::metaknight::*,
    smash::{
        app::{
            ArticleOperationTarget,
            AttackDirectionAxis,
            HitStatus,
            lua_bind::*,
            ShieldStatus,
            sv_animcmd::{
                frame,
                get_value_float,
                SET_TAKEOUT_SE_STATUS,
                wait
            }
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
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

mod beam_shoot;
mod special_hi_loop;
mod special_hi;
mod special_lw_attack;
mod special_lw_end;
mod special_lw_start;
mod special_n_end;
mod special_n;

pub fn install() {
    beam_shoot::install();
    special_hi_loop::install();
    special_hi::install();
    special_lw_attack::install();
    special_lw_end::install();
    special_lw_start::install();
    special_n_end::install();
    special_n::install();
}