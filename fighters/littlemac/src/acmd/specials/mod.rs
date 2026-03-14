use {
    exo_var::littlemac::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                get_value_float,
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod special_air_s_blow;
mod special_hi_jump;
mod special_lw_hit;
mod special_lw;
mod special_n;

pub fn install() {
    special_air_s_blow::install();
    special_hi_jump::install();
    special_lw_hit::install();
    special_lw::install();
    special_n::install();
}