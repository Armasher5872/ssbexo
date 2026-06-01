#![allow(internal_features)]
use {
    crate::{
        common::{
            extern_func::*,
            hook::*,
            ui_utility::*,
            vtable_funcs::*,
        },
        structs::{
            create_item_param::*,
            getter_funcs::*,
            stat_change::*,
            ui_manager::*,
            vector::*,
        }
    },
    exo_var::{
        armstrong::*,
        captain::*,
        cloud::*,
        consts::*,
        dedede::*,
        demon::*,
        donkey::*,
        edge::*,
        ganon::*,
        gaogaen::*,
        gekkouga::*,
        globals::*,
        inkling::*,
        ken::*,
        kirby::*,
        koopa::*,
        koopajr::*,
        krool::*,
        link::*,
        littlemac::*,
        luigi::*,
        mario::*,
        metaknight::*,
        miifighter::*,
        murabito::*,
        pikachu::*,
        roy::*,
        sheik::*,
        sonic::*,
        wario::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        *,
        macros::*
    },
    std::ffi::c_uint
};

pub mod armstrong;
pub mod captain;
pub mod cloud;
pub mod daisy;
pub mod dedede;
pub mod demon;
pub mod donkey;
pub mod edge;
pub mod ganon;
pub mod gaogaen;
pub mod gekkouga;
pub mod ice_climber_meter;
pub mod ike;
pub mod inkling;
pub mod ken;
pub mod kirby;
pub mod koopa;
pub mod koopajr;
pub mod krool;
pub mod link;
pub mod littlemac;
pub mod luigi;
pub mod mario;
pub mod mariod_meter;
pub mod metaknight;
pub mod miifighter;
pub mod murabito_shizue_common;
pub mod pfushigisou;
pub mod pikachu;
pub mod robot;
pub mod roy;
pub mod sheik;
pub mod shizue;
pub mod sonic;
pub mod wario;