use {
    exo_utils::structs::getter_funcs::*,
    exo_var::wario::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                get_value_float,
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

mod appeal_gas;
mod appeal_hi;
mod appeal_kamikaze;
mod appeal_lw;
mod dash;
mod escape_air;
mod run;
mod walk_fast;
mod walk_middle;
mod walk_slow;
mod win_1;
mod win_3;

pub fn install() {
    appeal_gas::install();
    appeal_hi::install();
    appeal_kamikaze::install();
    appeal_lw::install();
    dash::install();
    escape_air::install();
    run::install();
    walk_fast::install();
    walk_middle::install();
    walk_slow::install();
    win_1::install();
    win_3::install();
}