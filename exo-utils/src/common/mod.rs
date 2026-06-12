#![allow(internal_features)]
use {
    crate::{
        common::{
            extern_func::*,
            hook::*,
            state_manipulation::*,
        },
        status::damage::*,
        structs::{
            buttons::*,
            command_input_struct::*,
            getter_funcs::*,
            module_init::*,
            rect::*,
            shielddata_struct::*,
            stat_change::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        variables::*,
    },
    nnsdk::ui2d::{
        Pane,
        TextBox
    },
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
        phx::*
    },
    smash_script::macros::*
};

//A majority of the code found in these files originated from HDR, WuBor Patch, or Championship Edition
pub mod battle_object;
pub mod check_attack;
pub mod command_input;
pub mod extern_func;
pub mod fighter_common;
pub mod hook;
pub mod salty_runback;
pub mod status_end_control;
pub mod state_manipulation;
pub mod ui_utility;
pub mod ui2d;
pub mod var_reset;
pub mod vtable_funcs;
pub mod waza_customize;
pub mod weapon;