use {
    exo_utils::{
        attack_dash::*,
        attack_xx4::*,
        extern_func::*,
        fighter_common::*,
        link::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        kirby::*,
        link::*,
        murabito::*,
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
            Vector2f,
            Vector3f,
            Vector4f
        }
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::*,
};

mod arrow_fly;
mod arrow_haved;
mod arrow_hit_stick;
mod arrow_stick;
mod attack_dash_bound;
mod attack_dash;
mod attack_hi4_hold;
mod attack_hi4;
mod attack_lw4_hold;
mod attack_lw4;
mod attack_s4_hold;
mod attack_s4;
mod special_hi_ascend_end;
mod special_hi_ascend_start;
mod special_hi_ascend;
mod special_hi_drop;
mod special_hi_end;
mod special_hi_glide;
mod special_hi_land;
mod special_hi_turn;
mod special_hi;
mod special_lw_blast;
mod special_n;

pub fn install() {
    arrow_fly::install();
    arrow_haved::install();
    arrow_hit_stick::install();
    arrow_stick::install();
    attack_dash_bound::install();
    attack_dash::install();
    attack_hi4_hold::install();
    attack_hi4::install();
    attack_lw4_hold::install();
    attack_lw4::install();
    attack_s4_hold::install();
    attack_s4::install();
    special_hi_ascend_end::install();
    special_hi_ascend_start::install();
    special_hi_ascend::install();
    special_hi_drop::install();
    special_hi_end::install();
    special_hi_glide::install();
    special_hi_land::install();
    special_hi_turn::install();
    special_hi::install();
    special_lw_blast::install();
    special_n::install();
}