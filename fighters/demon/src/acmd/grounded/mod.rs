use {
    exo_var::{
        consts::*,
        demon::*,
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

mod attack_11_combo;
mod attack_11;
mod attack_12_combo;
mod attack_12;
mod attack_13;
mod attack_14;
mod attack_15;
mod attack_16;
mod attack_17;
mod attack_18;
mod attack_dash;
mod dash;
mod demon_god_fist;
mod demon_slayer;
mod devils_steel_pedal;
mod double_back_fist;
mod dragon_uppercut;
mod electric_dragon_uppercut;
mod electric_wind_god_fist;
mod escape;
mod jawbreaker;
mod left_splits_kick;
mod one_two_punch;
mod spinning_demon_to_hellsweep;
mod spinning_demon_to_left_hook;
mod twin_fang_double_kick;
mod twin_fang_stature_smash;
mod twin_pistons;
mod wind_god_fist;
mod zankyosho;

pub fn install() {
    attack_11_combo::install();
    attack_11::install();
    attack_12_combo::install();
    attack_12::install();
    attack_13::install();
    attack_14::install();
    attack_15::install();
    attack_16::install();
    attack_17::install();
    attack_18::install();
    attack_dash::install();
    dash::install();
    demon_god_fist::install();
    demon_slayer::install();
    devils_steel_pedal::install();
    double_back_fist::install();
    dragon_uppercut::install();
    electric_dragon_uppercut::install();
    electric_wind_god_fist::install();
    escape::install();
    jawbreaker::install();
    left_splits_kick::install();
    one_two_punch::install();
    spinning_demon_to_hellsweep::install();
    spinning_demon_to_left_hook::install();
    twin_fang_double_kick::install();
    twin_fang_stature_smash::install();
    twin_pistons::install();
    wind_god_fist::install();
    zankyosho::install();
}