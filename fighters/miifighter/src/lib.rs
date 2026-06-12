use {
    crate::status::{
        special_lw1_attack::*,
        special_lw1_charge::*,
        special_lw1::*,
        special_n3_dive::*,
        special_n3_land::*,
        special_n3_rise::*,
        special_n3::*,
        special_s1_end::*,
        special_s1::*,
    },
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            waza_customize::*,
        },
        fighter::miifighter::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        miifighter::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    },
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}