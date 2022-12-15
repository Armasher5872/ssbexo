use {
    crate::functions::{
        CHARGE_SHOT_TIMER,
        HAS_FIRE_CHARGE_SHOT,
        SAMUSD_CHECK_FLOAT,
        SAMUSD_FLOAT,
        SAMUSD_FLOAT_GFX_COUNTER,
        SAMUSD_FLOAT_MAX,
        SAMUSD_HAS_FLOAT,
        SAMUSD_START_FLOAT,
        SAMUSD_X,
        SAMUSD_X_ACCEL_MUL,
        SAMUSD_X_MAX,
        SAMUSD_Y,
        SAMUSD_Y_MAX,
        SITUATION_KIND
    },
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        phx::Hash40,
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        if sv_information::is_ready_go() == false {
            CHARGE_SHOT_TIMER[entry_id] = 0;
            HAS_FIRE_CHARGE_SHOT[entry_id] = false;
            SAMUSD_CHECK_FLOAT[entry_id] = 0;
            SAMUSD_FLOAT[entry_id] = 0;
            SAMUSD_START_FLOAT[entry_id] = false;
		};
        if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
                else {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        };
        if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) {
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
                else {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        };
        if CHARGE_SHOT_TIMER[entry_id] > 0 {
            CHARGE_SHOT_TIMER[entry_id] -= 1;
        }
        if CHARGE_SHOT_TIMER[entry_id] <= 0
        && HAS_FIRE_CHARGE_SHOT[entry_id] == true {
            HAS_FIRE_CHARGE_SHOT[entry_id] = false;
        }
        SAMUSD_FLOAT_GFX_COUNTER[entry_id] += 1;
        if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
            if SAMUSD_FLOAT_GFX_COUNTER[entry_id] > 25 {
                SAMUSD_FLOAT_GFX_COUNTER[entry_id] = 0;
            };
            if SAMUSD_START_FLOAT[entry_id] == true {
                if SAMUSD_FLOAT_GFX_COUNTER[entry_id] == 10 {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
                }
                if SAMUSD_FLOAT_GFX_COUNTER[entry_id] >= 20 {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
                    macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
                    SAMUSD_FLOAT_GFX_COUNTER[entry_id] = 0;
                }
            } 
            else {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
                macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
                macros::BURN_COLOR_NORMAL(fighter);
            };
        }
        if motion_kind == hash40("special_lw")
        && frame >= 30.0 {
            MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
        }
        if motion_kind == hash40("special_air_lw") {
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                SAMUSD_HAS_FLOAT[entry_id] = true;
            }
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            SAMUSD_HAS_FLOAT[entry_id] = false;
        }
        if SAMUSD_FLOAT[entry_id] == 1 {
            if KineticModule::get_kinetic_type(module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR
            && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            };
        };
        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR {
            if motion_kind == hash40("special_air_lw")
            && frame == 30.0 {
                SAMUSD_START_FLOAT[entry_id] = true;
            }
        };
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind)
        && SAMUSD_FLOAT[entry_id] > 1 {
            SAMUSD_FLOAT[entry_id] = 1;
        };
        if SAMUSD_FLOAT[entry_id] > 1 {
            SAMUSD_FLOAT[entry_id] -= 1;
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            };
            let mut y_add;
            let mut x_add;
            x_add = (stick_x)*SAMUSD_X_ACCEL_MUL;
            y_add = (stick_y)*SAMUSD_X_ACCEL_MUL;
            if x_add > 0.0 && SAMUSD_X[entry_id] > SAMUSD_X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && SAMUSD_X[entry_id] < SAMUSD_X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && SAMUSD_Y[entry_id] > SAMUSD_Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && SAMUSD_Y[entry_id] < SAMUSD_Y_MAX*-1.0 {
                y_add = 0.0;
            };
            SAMUSD_X[entry_id] += x_add;
            SAMUSD_Y[entry_id] += y_add;
            macros::SET_SPEED_EX(fighter, SAMUSD_X[entry_id], SAMUSD_Y[entry_id], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } 
        else {
            SAMUSD_X[entry_id] = 0.0;
            SAMUSD_Y[entry_id] = 0.0;
        };
        if SAMUSD_START_FLOAT[entry_id] == true {
            SAMUSD_FLOAT[entry_id] = SAMUSD_FLOAT_MAX;
            SAMUSD_START_FLOAT[entry_id] = false;
            if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            };
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
            };
            if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
            && SAMUSD_FLOAT[entry_id] > 1 {
                SAMUSD_FLOAT[entry_id] = 1;
            };
        };
    }
}

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        CHARGE_SHOT_TIMER[entry_id] = 0;
        HAS_FIRE_CHARGE_SHOT[entry_id] = false;
        SAMUSD_CHECK_FLOAT[entry_id] = 0;
        SAMUSD_FLOAT[entry_id] = 0;
        SAMUSD_START_FLOAT[entry_id] = false;
    }
}

pub fn install() {
    install_agent_frames!(
        samusd_frame
    );
    install_agent_resets!(
        fighter_reset
    );
}