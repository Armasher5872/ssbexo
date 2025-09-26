use {
    exo_utils::vector::*,
    exo_var::ganon::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
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

mod special_air_lw_end;
mod special_air_lw;
mod special_hi_end;
mod special_hi_move;
mod special_hi_start;
mod special_hi_throw;
mod special_lw;
mod special_n_cape;
mod special_n_fire_max;
mod special_n_fire;
mod special_n_loop;
mod special_n_start;
mod volley_fly_charge;
mod volley_fly;

pub fn install() {
    special_air_lw_end::install();
    special_air_lw::install();
    special_hi_end::install();
    special_hi_move::install();
    special_hi_start::install();
    special_hi_throw::install();
    special_lw::install();
    special_n_cape::install();
    special_n_fire_max::install();
    special_n_fire::install();
    special_n_loop::install();
    special_n_start::install();
    volley_fly_charge::install();
    volley_fly::install();
}