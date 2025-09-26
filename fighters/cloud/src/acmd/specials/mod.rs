use {
    exo_utils::{
        cloud::*,
        extern_func::*,
    },
    exo_var::cloud::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                get_value_float,
                LAST_EFFECT_SET_WORK_INT,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f
        }
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

mod special_hi_combo_1;
mod special_hi_combo_2_fall;
mod special_hi_combo_2_land;
mod special_hi_combo_2;
mod special_hi_lb;
mod special_hi;
mod special_lw_lb;
mod special_lw_punish_attack;
mod special_lw_punish;
mod special_lw;
mod special_n_lb;
mod special_s_lb;
mod special_s;

pub fn install() {
    special_hi_combo_1::install();
    special_hi_combo_2_fall::install();
    special_hi_combo_2_land::install();
    special_hi_combo_2::install();
    special_hi_lb::install();
    special_hi::install();
    special_lw_lb::install();
    special_lw_punish_attack::install();
    special_lw_punish::install();
    special_lw::install();
    special_n_lb::install();
    special_s_lb::install();
    special_s::install();
}