use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::misc::*,
        },
        var::{
            cloud::*,
            globals::*,
        }
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
mod attack_hi3;
mod attack_lw3;
mod attack_lw4_hold;
mod attack_lw4_start;
mod attack_lw4;
mod attack_s3;
mod dash;
mod escape_air;
mod escape_b;
mod escape_f;
mod escape;
mod fall;
mod final_smash;
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
mod special_lw;
mod special_n;
mod special_s;
mod special_s3;
mod squat_rv;
mod squat_wait;
mod squat;
mod turn;
mod wait;
mod walk;

pub fn install() {
    attack_air::install();
    attack_hi3::install();
    attack_lw3::install();
    attack_lw4_hold::install();
    attack_lw4_start::install();
    attack_lw4::install();
    attack_s3::install();
    dash::install();
    escape_air::install();
    escape_b::install();
    escape_f::install();
    escape::install();
    final_smash::install();
    fall::install();
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
    special_lw::install();
    special_n::install();
    special_s::install();
    special_s3::install();
    squat_rv::install();
    squat_wait::install();
    squat::install();
    turn::install();
    wait::install();
    walk::install();
}