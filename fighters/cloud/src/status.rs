use {
    exo_utils::{
        attack_dash::*,
        cloud::*,
        extern_func::*,
        ui_manager::*,
        vector::*,
    },
    exo_var::{
        cloud::*,
        consts::*,
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
        phx::Vector3f
    },
    smash_script::*,
    smashline::*,
};

mod attack_air;
mod attack_dash;
mod attack_hi3;
mod attack_hi4_hold;
mod attack_hi4_start;
mod attack_hi4;
mod attack_lw3;
mod attack_lw4_hold;
mod attack_lw4_start;
mod attack_lw4;
mod attack_s3;
mod attack_s4_hold;
mod attack_s4_start;
mod attack_s4;
mod attack;
mod cloud_guard_on;
mod cloud_guard_off;
mod cloud_guard;
mod counter_attack;
mod dash;
mod escape_b;
mod escape_f;
mod escape;
mod fall;
mod guard_on;
mod jump_aerial;
mod jump;
mod jumpsquat;
mod landing_attack_air;
mod run;
mod special_hi_combo_1;
mod special_hi_combo_2_fall;
mod special_hi_combo_2_land;
mod special_hi_combo_2;
mod special_hi_limit_break;
mod special_hi;
mod special_lw_limit_break;
mod special_lw;
mod special_n;
mod special_s_landing;
mod special_s_lb;
mod special_s;
mod squat_rv;
mod squat_wait;
mod squat;
mod turn;
mod wait;
mod walk;

pub fn install() {
    attack_air::install();
    attack_dash::install();
    attack_hi3::install();
    attack_hi4_hold::install();
    attack_hi4_start::install();
    attack_hi4::install();
    attack_lw3::install();
    attack_lw4_hold::install();
    attack_lw4_start::install();
    attack_lw4::install();
    attack_s3::install();
    attack_s4_hold::install();
    attack_s4_start::install();
    attack_s4::install();
    attack::install();
    cloud_guard_on::install();
    cloud_guard_off::install();
    cloud_guard::install();
    counter_attack::install();
    dash::install();
    escape_b::install();
    escape_f::install();
    escape::install();
    fall::install();
    guard_on::install();
    jump_aerial::install();
    jump::install();
    jumpsquat::install();
    landing_attack_air::install();
    run::install();
    special_hi_combo_1::install();
    special_hi_combo_2_fall::install();
    special_hi_combo_2_land::install();
    special_hi_combo_2::install();
    special_hi_limit_break::install();
    special_hi::install();
    special_lw_limit_break::install();
    special_lw::install();
    special_n::install();
    special_s_landing::install();
    special_s_lb::install();
    special_s::install();
    squat_rv::install();
    squat_wait::install();
    squat::install();
    turn::install();
    wait::install();
    walk::install();
}