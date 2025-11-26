use {
    exo_utils::{
        catch::*,
        check_attack::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        variables::*,
        wario::*,
    },
    smash::{
        app::{
            lua_bind::*,
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
        macros::*,
        *
    },
    smashline::*,
};

mod appeal_gas;
mod special_air_lw_end;
mod special_air_lw_land;
mod special_air_lw_loop;
mod special_air_s;
mod special_hi_jump;
mod special_lw_launch;
mod special_lw_loop;
mod special_lw;
mod special_n_catch_jump;
mod special_n_catch_turn;
mod special_n_catch_wait;
mod special_n_catch_walk;
mod special_n_catch;
mod special_n_throw_b;
mod special_n_throw_f;
mod special_n_throw_hi;
mod special_n_throw_lw_fall;
mod special_n_throw_lw_land;
mod special_n_throw_lw;
mod special_n;
mod special_s_air_loop;
mod special_s_end;
mod special_s_hit_end;
mod special_s_jumpsquat;
mod special_s_landing;
mod special_s_loop;
mod special_s_wall_end;
mod special_s;

pub fn install() {
    appeal_gas::install();
    special_air_lw_end::install();
    special_air_lw_land::install();
    special_air_lw_loop::install();
    special_air_s::install();
    special_hi_jump::install();
    special_lw_launch::install();
    special_lw_loop::install();
    special_lw::install();
    special_n_catch_jump::install();
    special_n_catch_turn::install();
    special_n_catch_wait::install();
    special_n_catch_walk::install();
    special_n_catch::install();
    special_n_throw_b::install();
    special_n_throw_f::install();
    special_n_throw_hi::install();
    special_n_throw_lw_fall::install();
    special_n_throw_lw_land::install();
    special_n_throw_lw::install();
    special_n::install();
    special_s_air_loop::install();
    special_s_end::install();
    special_s_hit_end::install();
    special_s_jumpsquat::install();
    special_s_landing::install();
    special_s_loop::install();
    special_s_wall_end::install();
    special_s::install();
}