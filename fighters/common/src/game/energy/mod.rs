use {
    exo_utils::{
        common::hook::*,
        structs::{
            buttons::*,
            energy_motion_reset_type::*,
            flydata::*,
            getter_funcs::*,
            kinetic_energy::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            BattleObjectModuleAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            *
        },
        hash40,
        lib::lua_const::*,
        phx::*
    }
};

mod control;
mod motion;

pub fn install() {
    control::install();
    motion::install();
}