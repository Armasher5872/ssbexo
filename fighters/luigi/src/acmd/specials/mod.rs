use {
    exo_utils::fighter_common::*,
    exo_var::luigi::*,
    smash::{
        app::{
            ArticleOperationTarget,
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Vector3f,
    },
    smash_script::{
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod fireball_regular;
mod special_hi;
mod special_lw_catch;
mod special_lw_end;
mod special_lw_loop;
mod special_lw_plunger;
mod special_lw_throw_b;
mod special_lw_throw_f;
mod special_lw_throw_hi;
mod special_lw_throw_plunger;
mod special_lw;
mod special_n_attack;

pub fn install() {
    fireball_regular::install();
    special_hi::install();
    special_lw_catch::install();
    special_lw_end::install();
    special_lw_loop::install();
    special_lw_plunger::install();
    special_lw_throw_b::install();
    special_lw_throw_f::install();
    special_lw_throw_hi::install();
    special_lw_throw_plunger::install();
    special_lw::install();
    special_n_attack::install();
}