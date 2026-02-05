#![feature(proc_macro_hygiene, repr_simd, simd_ffi, seek_stream_len)]
use {
    bitflags::bitflags,
    crate::{
        battle_object::*,
        cloud::*,
        create_item_param::*,
        damage::*,
        extern_func::*,
        fighter_common::*,
        hook::*,
        ice_climber_meter::*,
        knockback_calc_context::*,
        link::*,
        mariod_meter::*,
        rect::*,
        robot_meter::*,
        shielddata_struct::*,
        sonic::*,
        stat_change::*,
        ui_object::*,
        ui_utility::*,
        vector::*,
        vtable_funcs::*,
        weapon::*,
    },
    exo_var::{
        cloud::*,
        consts::*,
        ganon::*,
        globals::*,
        kirby::*,
        link::*,
        littlemac::*,
        murabito::*,
    },
    nnsdk::ui2d::{
        Pane,
        TextBox
    },
    once_cell::sync::Lazy,
    parking_lot::RwLock,
    skyline::libc::c_char,
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
        phx::{
            Hash40,
            Vector2f,
            Vector3f,
            Vector4f
        }
    },
    smash2::cpp::simd::*,
    smash_script::{
        *,
        macros::*
    },
    std::{
        ffi::c_uint,
        ops::{
            Deref,
            DerefMut
        }
    }
};

//A majority of the code found in these files originated from HDR, WuBor Patch, or Championship Edition
pub mod armstrong;
pub mod attack_dash;
pub mod attack_xx4;
pub mod attack;
pub mod attackinfo_struct;
pub mod battle_object;
pub mod buttons;
pub mod catch;
pub mod check_attack;
pub mod cloud;
pub mod collision_struct;
pub mod command_input;
pub mod create_item_param;
pub mod daisy;
pub mod damage;
pub mod donkey;
pub mod edge;
pub mod extern_func;
pub mod fighter_common;
pub mod flydata;
pub mod ganon;
pub mod gekkouga;
pub mod glide;
pub mod hook;
pub mod ice_climber_meter;
pub mod ike;
pub mod inkling;
pub mod kinetic_energy;
pub mod kirby;
pub mod knockback_calc_context;
pub mod knockback_func;
pub mod link;
pub mod littlemac;
pub mod mariod_meter;
pub mod metaknight;
pub mod miifighter;
pub mod murabito_shizue_common;
pub mod pfushigisou;
pub mod rect;
pub mod robot_meter;
pub mod sheik;
pub mod shielddata_struct;
pub mod sonic;
pub mod stat_change;
pub mod status_end_control;
pub mod ui_manager;
pub mod ui_object;
pub mod ui_utility;
pub mod ui2d;
pub mod vector;
pub mod vtable_funcs;
pub mod waza_customize;
pub mod weapon;