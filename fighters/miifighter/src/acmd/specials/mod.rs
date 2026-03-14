use {
    exo_var::{
        consts::*,
        miifighter::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod special_lw1_attack;
mod special_lw1_charge;
mod special_lw1;
mod special_lw3_toss;
mod special_lw3;
mod special_n3_dive;
mod special_n3_land;
mod special_n3_rise;
mod special_n3;
mod special_s1_end;
mod special_s3_catch;
mod special_s3_fall;
mod special_s3_throw;

pub fn install() {
    special_lw1_attack::install();
    special_lw1_charge::install();
    special_lw1::install();
    special_lw3_toss::install();
    special_lw3::install();
    special_n3_dive::install();
    special_n3_land::install();
    special_n3_rise::install();
    special_n3::install();
    special_s1_end::install();
    special_s3_catch::install();
    special_s3_fall::install();
    special_s3_throw::install();
}