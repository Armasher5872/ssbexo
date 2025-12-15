use {
    exo_utils::{
        edge::*,
        fighter_common::*,
    },
    exo_var::{
        edge::*,
        globals::*,
    },
    smash::{
        app::{
        lua_bind::*,
        *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Vector3f
    },
    smash_script::*,
    smashline::*,
};

mod fire_fly_l;
mod fire_fly_m;
mod fire_fly_s;
mod special_hi_end;
mod special_hi_landing;
mod special_n_shoot;
mod special_s_charge;

pub fn install() {
    fire_fly_l::install();
    fire_fly_m::install();
    fire_fly_s::install();
    special_hi_end::install();
    special_hi_landing::install();
    special_n_shoot::install();
    special_s_charge::install();
}