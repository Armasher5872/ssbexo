use {
    exo_utils::{
        fighter_common::*,
        weapon::*,
    },
    exo_var::link::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f
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

mod arrow_fly;
mod arrow_stick;
mod boomerang_fly;
mod boomerang_turn;
mod parasail_glide_drop;
mod parasail_glide_land;
mod parasail_glide;
mod parasail_special_hi;
mod special_hi_ascend_end;
mod special_hi_ascend_start;
mod special_hi_ascend;
mod special_hi_glide_drop;
mod special_hi_glide_land;
mod special_hi_glide;
mod special_hi_hold;
mod special_hi_spin_attack;
mod special_hi_start;
mod special_hi;

pub fn install() {
    arrow_fly::install();
    arrow_stick::install();
    boomerang_fly::install();
    boomerang_turn::install();
    parasail_glide_drop::install();
    parasail_glide_land::install();
    parasail_glide::install();
    parasail_special_hi::install();
    special_hi_ascend_end::install();
    special_hi_ascend_start::install();
    special_hi_ascend::install();
    special_hi_glide_drop::install();
    special_hi_glide_land::install();
    special_hi_glide::install();
    special_hi_hold::install();
    special_hi_spin_attack::install();
    special_hi_start::install();
    special_hi::install();
}