use {
    exo_utils::{
        fighter_common::*,
        metaknight::*,
        vector::*,
        weapon::*,
    },
    exo_var::{
        globals::*,
        metaknight::*,
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
    smashline::*,
    smash_script::*,
};

mod beam_shoot;
mod glide_attack;
mod glide_end;
mod glide_start;
mod special_hi;
mod special_lw_attack;
mod special_lw_end;
mod special_lw;

pub fn install() {
    beam_shoot::install();
    glide_attack::install();
    glide_end::install();
    glide_start::install();
    special_hi::install();
    special_lw_attack::install();
    special_lw_end::install();
    special_lw::install();
}