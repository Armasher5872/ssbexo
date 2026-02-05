use {
    exo_utils::{
        attack_xx4::*,
        check_attack::*,
        extern_func::*,
        fighter_common::*,
        link::*,
        vector::*,
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
            Vector3f
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
mod attach_wall;
mod attack_hi4_hold;
mod attack_hi4;
mod attack_lw4_hold;
mod attack_lw4;
mod attack_s4_hold;
mod attack_s4;
mod boomerang_fly;
mod boomerang_turn;
mod mortal_draw_attack;
mod mortal_draw_loop;
mod special_hi_drop;
mod special_hi_end;
mod special_hi_glide_start;
mod special_hi_glide;
mod special_hi_land;
mod special_hi_launch;
mod special_hi_loop;
mod special_hi;
mod special_lw_blast;
mod special_lw;
mod special_n;
mod special_s;
mod swordbeam_fly;

pub fn install() {
    arrow_fly::install();
    arrow_haved::install();
    arrow_hit_stick::install();
    arrow_stick::install();
    attach_wall::install();
    attack_hi4_hold::install();
    attack_hi4::install();
    attack_lw4_hold::install();
    attack_lw4::install();
    attack_s4_hold::install();
    attack_s4::install();
    boomerang_fly::install();
    boomerang_turn::install();
    mortal_draw_attack::install();
    mortal_draw_loop::install();
    special_hi_drop::install();
    special_hi_end::install();
    special_hi_glide_start::install();
    special_hi_glide::install();
    special_hi_land::install();
    special_hi_launch::install();
    special_hi_loop::install();
    special_hi::install();
    special_lw_blast::install();
    special_lw::install();
    special_n::install();
    special_s::install();
    swordbeam_fly::install();
}