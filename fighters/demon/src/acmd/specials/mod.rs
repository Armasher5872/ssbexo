use {
    exo_utils::structs::getter_funcs::*,
    exo_var::demon::*,
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
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod rage_drive;
mod special_hi;
mod special_lw;
mod special_n;
mod special_s;

pub fn install() {
    rage_drive::install();
    special_hi::install();
    special_lw::install();
    special_n::install();
    special_s::install();
}