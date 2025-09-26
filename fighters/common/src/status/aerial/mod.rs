use {
    exo_var::{
        bayonetta::*,
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
            *
        },
        hash40,
        lib::{
            L2CAgent,
            lua_const::*,
            *
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smashline::*,
    smash_script::*,
    std::f32::consts::PI
};

pub mod gliding;
mod sub_transition_group_check_air_attack;

pub fn install() {
    gliding::install();
    sub_transition_group_check_air_attack::install();
}