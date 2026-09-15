use {
    exo_var::{
        consts::*,
        roy::*,
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
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod special_hi;
mod special_lw_hit;
mod special_lw;
mod special_n_end_max;
mod special_n_end;
mod special_n_start;
mod special_n_turn;
mod special_s;

pub fn install() {
    special_hi::install();
    special_lw_hit::install();
    special_lw::install();
    special_n_end_max::install();
    special_n_end::install();
    special_n_start::install();
    special_n_turn::install();
    special_s::install();
}