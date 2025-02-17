use {
    crate::functions::var::{
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

mod fall;
pub mod gliding;

pub fn install() {
    fall::install();
    gliding::install();
}