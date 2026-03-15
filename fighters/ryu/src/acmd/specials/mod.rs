use {
    exo_var::ryu::*,
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

mod hadoken_move;
mod shakunetsu_move_last;
mod shakunetsu_move;
mod special_hi;
mod special_n;
mod special_s;

pub fn install() {
    hadoken_move::install();
    shakunetsu_move_last::install();
    shakunetsu_move::install();
    special_hi::install();
    special_n::install();
    special_s::install();
}