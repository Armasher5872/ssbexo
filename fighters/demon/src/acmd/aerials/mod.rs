use {
    exo_var::demon::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
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

mod bair;
mod dair;
mod fair;
mod high_pounce;
mod nair;

pub fn install() {
    bair::install();
    dair::install();
    fair::install();
    high_pounce::install();
    nair::install();
}