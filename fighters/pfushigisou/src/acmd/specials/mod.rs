use {
    exo_var::pfushigisou::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
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

mod sludge_shoot;
mod special_n;

pub fn install() {
    sludge_shoot::install();
    special_n::install();
}