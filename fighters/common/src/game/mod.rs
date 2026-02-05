use {
    exo_utils::{
        buttons::*,
        cloud::*,
        damage::*,
        extern_func::*,
        fighter_common::*,
        flydata::*,
        hook::*,
        ice_climber_meter::*,
        kinetic_energy::*,
        knockback_func::*,
        link::*,
        mariod_meter::*,
        robot_meter::*,
        sonic::*,
        ui_manager::*,
        ui_object::*,
        ui2d::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        donkey::*,
        ganon::*,
        gekkouga::*,
        globals::*,
        ike::*,
        link::*,
        metaknight::*,
        pfushigisou::*,
        variables::*,
    },
    param_config::*,
    skyline::{
        c_str,
        from_c_str,
        hooks::InlineCtx,
        nn::ui2d::*,
    },
    smash::{
        app::{
            BattleObjectModuleAccessor,
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
mod frame;
mod menu;
mod misc;
mod music;
mod ui;

pub fn install() {
    article::install();
    attack::install();
    control::install();
    delay::install();
    effect::install();
    energy::install();
    frame::install();
    menu::install();
    misc::install();
    music::install();
    ui::install();
}