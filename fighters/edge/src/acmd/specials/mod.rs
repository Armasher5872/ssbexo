use {
    exo_utils::{
        common::extern_func::*,
        status::attack::*,
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        edge::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                LAST_EFFECT_SET_WORK_INT,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod fire_burst_s;
mod fire_burst_xl;
mod fire_special_n4;
mod flare1_fly;
mod flare2_exp;
mod flash_attack;
mod special_hi2_end_wing;
mod special_lw_attack;
mod special_lw_end;
mod special_lw_loop;
mod special_lw_parry_hit;
mod special_lw_parry;
mod special_lw_flash;
mod special_lw_parry_flash;
mod special_lw_zanshin;
mod special_lw;
mod special_n_start_wing;
mod special_n_start;
mod special_n2;
mod special_n3;
mod special_s_cancel;
mod zanshin_shot_fly;
mod zanshin_shot_hit;
mod zanshin_shot_vanish;

pub fn install() {
    fire_burst_s::install();
    fire_burst_xl::install();
    fire_special_n4::install();
    flare1_fly::install();
    flare2_exp::install();
    flash_attack::install();
    special_hi2_end_wing::install();
    special_lw_attack::install();
    special_lw_end::install();
    special_lw_loop::install();
    special_lw_parry_hit::install();
    special_lw_parry::install();
    special_lw_flash::install();
    special_lw_parry_flash::install();
    special_lw_zanshin::install();
    special_lw::install();
    special_n_start_wing::install();
    special_n_start::install();
    special_n2::install();
    special_n3::install();
    special_s_cancel::install();
    zanshin_shot_fly::install();
    zanshin_shot_hit::install();
    zanshin_shot_vanish::install();
}