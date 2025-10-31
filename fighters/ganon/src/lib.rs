use {
    param_config::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
    },
    smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "ganon", "volley", false);
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_speed_max"), 0, 1.38));
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x"), 0, 0.802));
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x_max"), 0, 1.8));
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("landing_attack_air_frame_n"), 0, 8.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("landing_attack_air_frame_b"), 0, 10.0));
    update_int_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("attack_combo_max"), 0, 2)); //Vanilla Value is 1
    update_float_2(*FIGHTER_KIND_GANON, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("combo_attack_12_end"), 0, 37.0)); //Vanilla Value is 0
}