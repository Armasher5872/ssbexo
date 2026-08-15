use {
    exo_utils::{
        common::{
            check_attack::*,
            extern_func::*,
            hook::*,
        },
        fighter::edge::*,
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        edge::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    },
};

mod appeal;
mod attack_air;
mod attack_hi3;
mod attack_lw3;
mod attack_s4;
mod dash;
mod escape_air;
mod fire_burst_xl;
mod fire_fly_s;
mod fire_fly_xl;
mod flash_attack;
mod guard_on;
mod guard;
mod landing_attack_air;
mod run;
mod special_hi_charged_rush_winged;
mod special_hi_landing;
mod special_hi_rush;
mod special_lw_appeal;
mod special_lw_attack;
mod special_lw_end;
mod special_lw_hit;
mod special_lw_loop;
mod special_lw_parry_flash;
mod special_lw_parry;
mod special_lw_zanshin;
mod special_lw;
mod special_n_cancel;
mod special_n_shoot;
mod special_n;
mod special_s_cancel;
mod special_s_charge;
mod turn_dash;
mod turn_run_brake;
mod turn_run;
mod wing_activate;
mod zanshin_shot_fly;
mod zanshin_shot_hit;
mod zanshin_shot_vanish;

pub fn install() {
    appeal::install();
    attack_air::install();
    attack_hi3::install();
    attack_lw3::install();
    attack_s4::install();
    dash::install();
    escape_air::install();
    fire_burst_xl::install();
    fire_fly_s::install();
    fire_fly_xl::install();
    flash_attack::install();
    guard_on::install();
    guard::install();
    landing_attack_air::install();
    run::install();
    special_hi_charged_rush_winged::install();
    special_hi_landing::install();
    special_hi_rush::install();
    special_lw_appeal::install();
    special_lw_attack::install();
    special_lw_end::install();
    special_lw_hit::install();
    special_lw_loop::install();
    special_lw_parry_flash::install();
    special_lw_parry::install();
    special_lw_zanshin::install();
    special_lw::install();
    special_n_cancel::install();
    special_n_shoot::install();
    special_n::install();
    special_s_cancel::install();
    special_s_charge::install();
    turn_dash::install();
    turn_run_brake::install();
    turn_run::install();
    wing_activate::install();
    zanshin_shot_fly::install();
    zanshin_shot_hit::install();
    zanshin_shot_vanish::install();
}