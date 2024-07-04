#![allow(unused_parens, unused_variables, unused_unsafe)]
use {
    crate::functions::{
        ext::{
            fighter::{
                common::*,
                link::*,
            },
            utility::{
                boma_ext::*,
                commandcat::*,
                controls::*,
                misc::*,
                get_objects::*,
            }
        },
        var::{
            consts::*,
            globals::*,
            pfushigisou::*,
            purin::*,
            variables::*,
        },
        util::*,
    },
    skyline::{
        c_str,
        from_c_str,
        hooks::InlineCtx
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectModuleAccessor,
            FighterUtil,
            GroundCorrectKind,
            HitStatus,
            lua_bind::{
                PostureModule,
                *
            },
            sv_animcmd,
            utility::*,
        },
        hash40,
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Vector3f
    },
    std::{
        ffi::CStr,
        os::raw::{
            c_char,
            c_int
        }
    }
};

pub mod attack;
pub mod controls;
pub mod effect;
pub mod fighter;
pub mod misc;
pub mod music;
pub mod params;
pub mod status;
pub mod throw;
pub mod workmodule;

pub fn install() {
    attack::install();
    controls::install();
    effect::install();
    fighter::install();
    misc::install();
    music::install();
    params::install();
    status::install();
    throw::install();
    workmodule::install();
}