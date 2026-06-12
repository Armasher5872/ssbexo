#![allow(internal_features)]
use {
    bitflags::bitflags,
    crate::{
        common::{
            battle_object::*,
            extern_func::*,
            ui_utility::*,
        },
        fighter::{
            cloud::*,
            ice_climber_meter::*,
            link::*,
            mariod_meter::*,
            robot::*,
            sonic::*,
        },
        structs::{
            buttons::*,
            energy_motion_reset_type::*,
            ui_object::*,
            vector::*,
        }
    },
    exo_var::consts::*,
    modular_bitfield::{
        bitfield,
        specifiers::*,
    },
    once_cell::sync::Lazy,
    parking_lot::RwLock,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash2::cpp::simd::*,
    std::ops::{
        Deref,
        DerefMut
    }
};

pub mod buttons;
pub mod collision_struct;
pub mod command_input_struct;
pub mod controller_struct;
pub mod energy_motion_reset_type;
pub mod flydata;
pub mod getter_funcs;
pub mod hashed_string;
pub mod kinetic_energy;
pub mod knockback_calc_context;
pub mod module_init;
pub mod rect;
pub mod shielddata_struct;
pub mod stat_change;
pub mod ui_manager;
pub mod ui_object;
pub mod vector;