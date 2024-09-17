#![allow(dead_code)]
use {
    crate::functions::{
        ext::utility::misc::*,
        hook::misc::*,
        var::{
            consts::*,
            donkey::*,
            globals::*,
            ike::*,
            kirby::*,
            link::*,
            metaknight::*,
            murabito::*,
            purin::*,
        }
    },
    smash::{
        app::{
            ArticleOperationTarget,
            BattleObjectModuleAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector2f,
            Vector3f,
            Vector4f
        }
    },
    smash_script::*,
    std::os::raw::c_char
};

pub mod common;
pub mod donkey;
pub mod ike;
pub mod inkling;
pub mod link;
pub mod metaknight;
pub mod murabito_shizue_common;
pub mod purin;