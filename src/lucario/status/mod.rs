use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            misc::*,
        },
        var::{
            globals::*,
            lucario::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod dashing_force_palm;
mod high_jump_kick_landing;
mod high_jump_kick_start;
mod high_jump_kick;
mod lucariom;
mod mega_evolve;
mod power_up_punch;
mod special_lw;
mod special_s_hi;
mod special_s_lw;
mod special_s;

pub fn install() {
    dashing_force_palm::install();
    high_jump_kick_landing::install();
    high_jump_kick_start::install();
    high_jump_kick::install();
    lucariom::install();
    mega_evolve::install();
    power_up_punch::install();
    special_lw::install();
    special_s_hi::install();
    special_s_lw::install();
    special_s::install();
}