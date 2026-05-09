use {
    exo_utils::{
        common::{
            extern_func::*,
            hook::*,
            ui2d::*,
        },
        fighter::{
            cloud::*,
            ice_climber_meter::*,
            link::*,
            mariod_meter::*,
            robot::*,
            sonic::*,
        },
        status::{
            damage::*,
            knockback_func::*,
        },
        structs::{
            buttons::*,
            flydata::*,
            getter_funcs::*,
            kinetic_energy::*,
            ui_manager::*,
            ui_object::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        donkey::*,
        ganon::*,
        gekkouga::*,
        globals::*,
        ike::*,
        lucario::*,
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
mod command_user;
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
    command_user::install();
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