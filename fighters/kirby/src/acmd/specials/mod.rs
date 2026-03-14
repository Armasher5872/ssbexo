use {
    exo_var::kirby::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
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

mod special_air_lw;
mod special_hi_1;
mod special_hi_2;
mod special_hi_3;
mod special_hi_4;
mod special_hi_5;
mod special_lw_landing;
mod special_s;

pub fn install() {
    special_air_lw::install();
    special_hi_1::install();
    special_hi_2::install();
    special_hi_3::install();
    special_hi_4::install();
    special_hi_5::install();
    special_lw_landing::install();
    special_s::install();
}