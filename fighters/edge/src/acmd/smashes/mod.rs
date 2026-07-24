use {
    exo_var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
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

mod attack_hi4;
mod attack_lw4;
mod attack_s4_hi;
mod attack_s4_lw1;
mod attack_s4_lw2;

pub fn install() {
    attack_hi4::install();
    attack_lw4::install();
    attack_s4_hi::install();
    attack_s4_lw1::install();
    attack_s4_lw2::install();
}