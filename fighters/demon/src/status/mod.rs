use {
    exo_utils::{
        common::extern_func::*,
        fighter::demon::*,
        status::catch::*,
        structs::buttons::*,
    },
    exo_var::{
        demon::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
        phx::*,
    },
    smashline::*,
    smash_script::*,
};

mod attack_combo;
mod attack_dash_2;
mod attack_dash_3;
mod attack_lw3_cancel_attack;
mod attack_lw3_cancel;
mod attack_lw3_edge;
mod attack_rage;
mod attack_squat_4;
mod attack_stand_5;
mod attack_stand_7;
mod attack_step_2;
mod attack_step_2f;
mod attack_step_2h;
mod attack_step_2k;
mod attack_step_2l;
mod attack_step_2s;
mod attack_step;
mod attack;
mod catch_back;
mod dash;
mod escape_attack;
mod escape;
mod flash_punch;
mod landing_attack_air;
mod landing_light;
mod landing;
mod special_lw;
mod special_n_air_shoot;
mod special_n_air_start;
mod special_n_ground_shoot;
mod special_n_ground_start;

pub fn install() {
    attack_combo::install();
    attack_dash_2::install();
    attack_dash_3::install();
    attack_lw3_cancel_attack::install();
    attack_lw3_cancel::install();
    attack_lw3_edge::install();
    attack_rage::install();
    attack_squat_4::install();
    attack_stand_5::install();
    attack_stand_7::install();
    attack_step_2::install();
    attack_step_2f::install();
    attack_step_2h::install();
    attack_step_2k::install();
    attack_step_2l::install();
    attack_step_2s::install();
    attack_step::install();
    attack::install();
    catch_back::install();
    dash::install();
    escape_attack::install();
    escape::install();
    flash_punch::install();
    landing_attack_air::install();
    landing_light::install();
    landing::install();
    special_lw::install();
    special_n_air_shoot::install();
    special_n_air_start::install();
    special_n_ground_shoot::install();
    special_n_ground_start::install();
}