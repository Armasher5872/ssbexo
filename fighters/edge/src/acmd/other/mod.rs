use {
    exo_var::{
        consts::*,
        edge::*,
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

mod appeal_s;
mod dash_wing;
mod dash;
mod escape_air;
mod run_wing;
mod run;
mod turn_dash_wing;
mod turn_dash;
mod turn_run_brake_wing;
mod turn_run_brake;
mod turn_run_wing;
mod turn_run;
mod win_3;

pub fn install() {
    appeal_s::install();
    dash_wing::install();
    dash::install();
    escape_air::install();
    run_wing::install();
    run::install();
    turn_dash_wing::install();
    turn_dash::install();
    turn_run_brake_wing::install();
    turn_run_brake::install();
    turn_run_wing::install();
    turn_run::install();
    win_3::install();
}