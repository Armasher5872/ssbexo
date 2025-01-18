//Most of the code in this folder is credited to either HDR's Code Repository, or Championship Edition
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
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    std::ops::{
        Deref,
        DerefMut
    }
};

pub mod boma_ext;
pub mod commandcat;
pub mod controls;
pub mod get_objects;
pub mod misc;