use {
    exo_utils::{
        armstrong::*,
        battle_object::*,
        buttons::*,
        cloud_meter::*,
        extern_func::*,
        fighter_common::*,
        flydata::*,
        hook::*,
        ice_climber_meter::*,
        kinetic_energy::*,
        lucario_meter::*,
        mariod_meter::*,
        robot_meter::*,
        palutena_meter::*,
        ui_manager::*,
        ui_object::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        purin::*,
        variables::*,
    },
    param_config::*,
    skyline::{
        c_str,
        from_c_str,
        hooks::InlineCtx
    },
    smash::{
        app::{
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
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::macros::*,
    std::{
        ffi::CStr,
        os::raw::c_char,
        sync::atomic::{
            AtomicBool,
            Ordering
        }
    }
};

mod article;
mod attack;
mod control;
mod delay;
mod effect;
mod energy;
//mod fighterutil;
mod frame;
mod menu;
pub mod misc;
mod music;
//mod parammodule;
mod status;
mod ui;
//mod workmodule;

pub fn install() {
    article::install();
    attack::install();
    control::install();
    delay::install();
    effect::install();
    energy::install();
    //fighterutil::install();
    frame::install();
    menu::install();
    misc::install();
    music::install();
    //parammodule::install();
    status::install();
    ui::install();
    //workmodule::install();
}