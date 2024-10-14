use {
    crate::{
        functions::{
            ext::utility::boma_ext::*,
            var::{
                globals::*,
                rockman::*,
            }
        },
        rockman::status::helper::*,
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
        lua2cpp::L2CFighterCommon
    },
    smash_script::*,
    smashline::*,
};

mod attack_air;
mod attack_s4;
pub mod helper;
mod ladder_attack;
mod rockbuster_shoot_air;
mod rockbuster_shoot_jump;
mod rockbuster_shoot_jumpsquat;
mod rockbuster_shoot_landing;
mod rockbuster_shoot_turn;
mod rockbuster_shoot_wait;
mod rockbuster_shoot_walk;
mod special_lw;
mod special_n;

pub fn install() {
    attack_air::install();
    attack_s4::install();
    ladder_attack::install();
    rockbuster_shoot_air::install();
    rockbuster_shoot_jump::install();
    rockbuster_shoot_jumpsquat::install();
    rockbuster_shoot_landing::install();
    rockbuster_shoot_turn::install();
    rockbuster_shoot_wait::install();
    rockbuster_shoot_walk::install();
    special_lw::install();
    special_n::install();
}