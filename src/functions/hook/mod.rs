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
        hook::misc::*,
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
            *
        },
        hash40,
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        },
        phx::{
            Hash40,
            Vector3f,
            Vector2f
        }
    },
    smash_script::*,
    std::{
        arch::asm,
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
pub mod energy;
pub mod fighter;
pub mod jump;
pub mod knockback;
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
    effect::install();
    energy::install();
    fighter::install();
    jump::install();
    knockback::install();
    menu::install();
    misc::install();
    music::install();
    params::install();
    status::install();
    ui::install();
    weapon::install();
    workmodule::install();
}