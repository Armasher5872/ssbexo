use {
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

mod crouch_jab;
mod crouch_spin_kick;
mod demons_wrath;
mod fujin_uraken;
mod high_kick_jab;
mod hook;
mod jump_side_kick;
mod nejiri_uraken_cancel;
mod nejiri_uraken;
mod oni_stomp;
mod reign_of_terror;
mod tombstone_crusher;
mod tsunami_kick;

pub fn install() {
    crouch_jab::install();
    crouch_spin_kick::install();
    demons_wrath::install();
    fujin_uraken::install();
    high_kick_jab::install();
    hook::install();
    jump_side_kick::install();
    nejiri_uraken_cancel::install();
    nejiri_uraken::install();
    oni_stomp::install();
    reign_of_terror::install();
    tombstone_crusher::install();
    tsunami_kick::install();
}