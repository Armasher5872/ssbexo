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
            cloud::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
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
mod guard;
pub mod jump;
pub mod knockback;
mod ledge;
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
    guard::install();
    jump::install();
    knockback::install();
    ledge::install();
    menu::install();
    misc::install();
    music::install();
    params::install();
    status::install();
    ui::install();
    weapon::install();
    workmodule::install();
}