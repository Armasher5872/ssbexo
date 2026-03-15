use {
    exo_utils::{
        armstrong::*,
        fighter_common::*,
        vector::*,
    },
    exo_var::armstrong::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                RUMBLE_HIT_STATUS,
                START_INFO_FLASH_EYE,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
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

mod final_start;
mod final_throw;
mod firepillar_burst;
mod special_hi_catch;
mod special_hi_throw;
mod special_hi;
mod special_lw;
mod special_n;
mod special_s_catch;
mod special_s_end;
mod special_s_fall;
mod special_s_run;
mod special_s_start;
mod special_s_throw;

pub fn install() {
    final_start::install();
    final_throw::install();
    firepillar_burst::install();
    special_hi_catch::install();
    special_hi_throw::install();
    special_hi::install();
    special_n::install();
    special_lw::install();
    special_s_catch::install();
    special_s_end::install();
    special_s_fall::install();
    special_s_run::install();
    special_s_start::install();
    special_s_throw::install();
}