use {
    exo_utils::sonic::*,
    exo_var::{
        consts::*,
        globals::*,
        sonic::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod attack_air;
mod attack_dash;
mod attack_hi3;
mod attack_hi4_start;
mod attack_hi4;
mod attack_lw3;
mod attack_lw4_start;
mod attack_lw4;
mod attack_s3;
mod attack_s4_start;
mod attack_s4;
mod attack;
mod special_lw_hold;
mod special_lw;
mod special_n_cancel;
mod special_n_fail;
mod special_n_hit;
mod special_n_rebound;
mod special_n;
mod special_s_dash;
mod special_s_rush;
mod special_s_rush_end;
mod special_s;
mod throw;

pub fn install() {
    attack_air::install();
    attack_dash::install();
    attack_hi3::install();
    attack_hi4_start::install();
    attack_hi4::install();
    attack_lw3::install();
    attack_lw4_start::install();
    attack_lw4::install();
    attack_s3::install();
    attack_s4_start::install();
    attack_s4::install();
    attack::install();
    special_lw_hold::install();
    special_lw::install();
    special_n_cancel::install();
    special_n_fail::install();
    special_n_hit::install();
    special_n_rebound::install();
    special_n::install();
    special_s_dash::install();
    special_s_rush::install();
    special_s_rush_end::install();
    special_s::install();
    throw::install();
}