use {
    exo_var::globals::*,
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

pub fn install() {
    gliding::install();
}