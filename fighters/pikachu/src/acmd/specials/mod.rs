use {
    exo_utils::structs::getter_funcs::*,
    exo_var::pikachu::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                EFFECT_FLW_POS_UNSYNC_VIS,
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

mod special_hi;
mod special_s;

pub fn install() {
    special_hi::install();
    special_s::install();
}