use {
    exo_utils::{
        check_attack::*,
        fighter_common::*,
        hook::*,
        shielddata_struct::*,
        status_end_control::*,
        vector::*,
        vtable_funcs::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        ness::*,
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
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    },
    smash_script::{
        *,
        macros::*
    },
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