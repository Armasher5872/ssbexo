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
            }
        },
        var::{
            consts::*,
            globals::*,
            palutena::*,
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
        phx::Vector3f
    },
    std::{
        ffi::CStr,
        os::raw::{
            c_char,
            c_int
        },
        sync::atomic::{
            AtomicBool,
            Ordering
        }
    }
};

pub mod attack;
pub mod controls;
pub mod delay;
pub mod effect;
pub mod fighter;
pub mod menu;
pub mod misc;
pub mod music;
pub mod params;
pub mod status;
pub mod ui;
pub mod weapon;
pub mod workmodule;

pub fn install() {
    attack::install();
    controls::install();
    delay::install();
    effect::install();
    fighter::install();
    menu::install();
    misc::install();
    music::install();
    params::install();
    status::install();
    ui::install();
    weapon::install();
    workmodule::install();
}