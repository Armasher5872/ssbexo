use {
    exo_utils::{
        common::{
            extern_func::*,
            fighter_common::*,
            hook::*,
            salty_runback::*,
            ui2d::*,
        },
        fighter::{
            armstrong::*,
            cloud::*,
            edge::*,
            ice_climber_meter::*,
            link::*,
            mariod_meter::*,
            robot::*,
            sonic::*,
        },
        status::knockback_func::*,
        structs::{
            getter_funcs::*,
            ui_manager::*,
            ui_object::*,
        }
    },
    exo_var::{
        armstrong::*,
        consts::*,
        donkey::*,
        edge::*,
        ganon::*,
        gekkouga::*,
        globals::*,
        ike::*,
        lucario::*,
        metaknight::*,
        pfushigisou::*,
        pikachu::*,
        variables::*,
        wario::*,
    },
    ninput::any::*,
    param_config::*,
    rand::Rng,
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
        phx::*
    }
};

mod article;
mod attack;
mod command_user;
mod control;
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
    effect::install();
    energy::install();
    frame::install();
    menu::install();
    misc::install();
    music::install();
    ui::install();
}