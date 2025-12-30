use {
    exo_var::{
        consts::*,
        ganon::*,
        globals::*,
    },
    exo_utils::{
        fighter_common::*,
        ganon::*,
        vector::*,
        weapon::*,
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

mod appeal_attack;
mod appeal;
mod catch_pull;
mod catch_wait;
mod special_hi_cling;
mod special_hi_end;
mod special_hi_move;
mod special_hi_start;
mod special_hi_throw;
mod special_n_cape;
mod special_n_fire;
mod special_n_loop;
mod special_n;
mod volley_fly;
mod volley_summon;

pub fn install() {
    appeal_attack::install();
    appeal::install();
    catch_pull::install();
    catch_wait::install();
    special_hi_cling::install();
    special_hi_end::install();
    special_hi_move::install();
    special_hi_start::install();
    special_hi_throw::install();
    special_n_cape::install();
    special_n_fire::install();
    special_n_loop::install();
    special_n::install();
    volley_fly::install();
    volley_summon::install();
}