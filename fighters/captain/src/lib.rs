use {
    exo_utils::{
        fighter_common::*,
        status_end_control::*,
    },
    exo_var::{
        captain::*,
        consts::*,
        globals::*,
        variables::*,
    },
    smash::{
        app::{
            BattleObjectModuleAccessor,
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smash_script::macros::*,
    smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
}