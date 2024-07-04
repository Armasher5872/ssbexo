//Most of the code in this folder is credited to either HDR's Code Repository, or Championship Edition
#![allow(dead_code, non_snake_case, non_upper_case_globals)]
use {
    bitflags::bitflags,
    crate::functions::{
        ext::utility::commandcat::*,
        hook::misc::*,
        var::globals::*,
    },
    modular_bitfield::{
        bitfield,
        specifiers::B4
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectModuleAccessor,
            Fighter,
            lua_bind::{
                PostureModule,
                *
            },
            sv_battle_object,
            utility::*,
        },
        hash40,
        lib::{
            L2CAgent,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
};

pub mod boma_ext;
pub mod commandcat;
pub mod controls;
pub mod frame_info;
pub mod get_objects;
pub mod misc;