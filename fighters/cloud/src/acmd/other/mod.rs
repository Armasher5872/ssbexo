use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
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

mod appeal_lw;
mod escape_air;
mod punisher_dash;
mod punisher_guard_off;
mod punisher_guard_on;
mod punisher_guard;
mod punisher_jump_aerial_back;
mod punisher_jump_aerial_front;
mod punisher_jump_back_mini;
mod punisher_jump_back;
mod punisher_jump_front_mini;
mod punisher_jump_front;
mod punisher_jumpsquat;
mod punisher_squat_rv;
mod punisher_squat_wait;
mod punisher_squat;
mod punisher_turn;
mod punisher_walk_fast;
mod punisher_walk_middle;
mod punisher_walk_slow;

pub fn install() {
    appeal_lw::install();
    escape_air::install();
    punisher_dash::install();
    punisher_guard_off::install();
    punisher_guard_on::install();
    punisher_guard::install();
    punisher_jump_aerial_back::install();
    punisher_jump_aerial_front::install();
    punisher_jump_back_mini::install();
    punisher_jump_back::install();
    punisher_jump_front_mini::install();
    punisher_jump_front::install();
    punisher_jumpsquat::install();
    punisher_squat_rv::install();
    punisher_squat_wait::install();
    punisher_squat::install();
    punisher_turn::install();
    punisher_walk_fast::install();
    punisher_walk_middle::install();
    punisher_walk_slow::install();
}