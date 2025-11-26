use {
    exo_utils::{
        donkey::*,
        fighter_common::*,
    },
    exo_var::{
        consts::*,
        donkey::*,
        globals::*,
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
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod air_lasso_landing;
mod air_lasso;
mod barrel_break;
mod barrel_idle;
mod barrel_pull;
mod barrel_roll;
mod catch_pull;
mod shoulder_start;
mod special_hi;
mod special_lw;
mod special_n;
mod special_s;

pub fn install() {
    air_lasso_landing::install();
    air_lasso::install();
    barrel_break::install();
    barrel_idle::install();
    barrel_pull::install();
    barrel_roll::install();
    catch_pull::install();
    shoulder_start::install();
    special_hi::install();
    special_lw::install();
    special_n::install();
    special_s::install();
}