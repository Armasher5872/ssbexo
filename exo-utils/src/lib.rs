#![feature(concat_idents, proc_macro_hygiene, repr_simd, simd_ffi, seek_stream_len)]
use {
    bitflags::bitflags,
    crate::{
        battle_object::*,
        cloud_meter::*,
        common_ui_enable_disable::*,
        create_item_param::*,
        donkey::*,
        extern_func::*,
        fighter_common::*,
        gekkouga::*,
        hook::*,
        ice_climber_meter::*,
        ike::*,
        lucario_meter::*,
        mariod_meter::*,
        metaknight::*,
        palutena_meter::*,
        pfushigisou::*,
        purin::*,
        rect::*,
        robot_meter::*,
        shielddata_struct::*,
        ui_manager::*,
        ui_object::*,
        ui_utility::*,
        vector::*,
        weapon::*,
    },
    exo_var::{
        cloud::*,
        consts::*,
        donkey::*,
        globals::*,
        kirby::*,
        link::*,
        littlemac::*,
        murabito::*,
        rockman::*,
        variables::*,
    },
    once_cell::sync::Lazy,
    parking_lot::RwLock,
    smash::{
        app::{
            lua_bind::*,
            utility::*,
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
        os::raw::c_char,
        ops::{
            Deref,
            DerefMut
        }
    }
};

//A majority of the code found in these files originated from HDR, WuBor Patch, or Championship Edition
pub mod attack_dash;
pub mod attack_xx4;
pub mod attack;
pub mod attackinfo_struct;
pub mod battle_object;
pub mod buttons;
pub mod catch;
pub mod check_attack;
pub mod cloud_meter;
pub mod cloud;
pub mod collision_struct;
pub mod command_input;
pub mod common_ui_enable_disable;
pub mod create_item_param;
pub mod daisy;
pub mod damage;
pub mod donkey;
pub mod extern_func;
pub mod fighter_common;
pub mod flydata;
pub mod gekkouga;
pub mod hook;
pub mod ice_climber_meter;
pub mod ike;
pub mod inkling;
pub mod kinetic_energy;
pub mod knockback;
pub mod link;
pub mod littlemac;
pub mod lucario_meter;
pub mod mariod_meter;
pub mod metaknight;
pub mod murabito_shizue_common;
pub mod palutena_meter;
pub mod pfushigisou;
pub mod purin;
pub mod rect;
pub mod robot_meter;
pub mod rockman;
pub mod shielddata_struct;
pub mod status_end_control;
pub mod ui_manager;
pub mod ui_object;
pub mod ui_utility;
pub mod vector;
pub mod vtable_funcs;
pub mod waza_customize;
pub mod weapon;