#![allow(unused_assignments)]
use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::boma_ext::*,
        },
        var::{
            consts::*,
            globals::*,
            luigi::*,
        }
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
            Vector2f
        }
    },
    smashline::*,
};

mod air_lasso;
mod air_lasso_landing;
mod catch;
mod catch_attack;
mod catch_cut;
mod catch_dash;
mod catch_dash_pull;
mod catch_pull;
mod catch_turn;
mod catch_wait;
mod special_lw;
mod special_lw_end;
mod special_lw_loop;
mod special_s;
mod special_s_breath;
mod special_s_charge;
mod special_s_end;
mod special_s_ram;
mod special_s_wall;
mod throw;

pub fn install() {
    air_lasso::install();
    air_lasso_landing::install();
    catch::install();
    catch_attack::install();
    catch_cut::install();
    catch_dash::install();
    catch_dash_pull::install();
    catch_pull::install();
    catch_turn::install();
    catch_wait::install();
    special_lw::install();
    special_lw_end::install();
    special_lw_loop::install();
    special_s::install();
    special_s_breath::install();
    special_s_charge::install();
    special_s_end::install();
    special_s_ram::install();
    special_s_wall::install();
    throw::install();
}