use super::*;

unsafe extern "C" fn sonic_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let rand_num_8 = sv_math::rand(hash40("fighter"), 8);
    let rand_num_10 = sv_math::rand(hash40("fighter"), 10);
    let sonic_new_animation_hash = Hash40::new(match rand_num_8 {1|2 => "special_n_hit", 3..=4 => "special_n_hit_1", 5..=6 => "special_n_hit_2", _ => "special_n_hit_3"});
    let parried = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    let parry_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    //Effect Clearing 
    if ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        EffectModule::kill_kind(module_accessor, Hash40::new("sonic_spintrace_homing"), false, true);
        EffectModule::kill_kind(module_accessor, Hash40::new("sonic_spintrace_middle"), false, true);
        EffectModule::kill_kind(module_accessor, Hash40::new("sys_shield"), false, true);
    };
    //Boost Speed Tracking
    if ![*FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        SONIC_BOOST_SPEED[entry_id] = KineticModule::get_sum_speed_x(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        SONIC_BOOST_SPEED[entry_id] *= PostureModule::lr(module_accessor);
    };
    //Boost Attack Addition Check
    if frame < 2.0
    && ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) { // resets at the start of a move the inability to add further boost charge
        WorkModule::set_flag(module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    };
    if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD)
    && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) 
    && ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        WorkModule::set_flag(module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        SONIC_BOOST[entry_id] += 1.0;
    };
    //Boost Tally Removal/Prevention
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if SONIC_BOOST[entry_id] < 25.0
        && frame < 1.0 {
            WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, true);
            macros::SET_SPEED_EX(fighter, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if SONIC_BOOST[entry_id] >= 25.0
        && frame > 5.0 {
            SONIC_BOOST[entry_id] = 0.0;
        }
    }
    //Wait
    if status_kind == *FIGHTER_STATUS_KIND_WAIT {
        if motion_kind == hash40("wait_2")
        && frame == 40.0 
        && rand_num_10 <= 2 {
            MotionModule::change_motion(module_accessor, Hash40::new("wait_2_a"), 1.0, 1.0, false, 0.0, false, false);
        };
        if motion_kind == hash40("wait_2_a")
        && frame >= 157.0 {
            MotionModule::change_motion(module_accessor, Hash40::new("wait_2_b"), 1.0, 1.0, false, 0.0, false, false);
        };
        if motion_kind == hash40("wait_2_b")
        && frame >= 38.0 {
            MotionModule::change_motion(module_accessor, Hash40::new("wait_2_b"), 1.0, 1.0, false, 0.0, false, false);
        };
    };
    //Taunt Loops
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind)
        && frame >= 45.0 {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)  {
                MotionModule::set_frame_sync_anim_cmd(module_accessor, 29.0, true, true, false);
            };
        }
        if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && frame >= 53.0 {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                MotionModule::set_frame_sync_anim_cmd(module_accessor, 13.0, true, true, false);
            };
        }
    };
    //Parry Timer 
    if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind)
    && (0.0..5.0).contains(&frame)
    && parried != 1 {
        WorkModule::set_int(module_accessor, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(module_accessor, 180, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        SONIC_BOOST[entry_id] += 5.0;
    }
    if parry_timer > 0 {
        WorkModule::dec_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    }
    if parry_timer <= 0
    && parried == 1 {
        WorkModule::set_int(module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    }
    //Jab Cancel
    if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind)
    && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
        if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
        } 
        else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
        } 
        else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
        };
    };
    //Jab 2
    if motion_kind == hash40("attack_12")
    && (3.0..8.0).contains(&frame)
    && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        MotionModule::change_motion(module_accessor, Hash40::new("attack_100"), 1.0, 1.0, false, 0.0, false, false);
    }
    //Rapid Jab
    if motion_kind == hash40("attack_100") {
        if frame >= 1.0 {
            AttackModule::clear_all(module_accessor);
            if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(module_accessor, Hash40::new("attack_13"), 1.0, 1.0, false, 0.0, false, false);
            }
        };
        if frame >= 17.0
        && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::set_frame_sync_anim_cmd(module_accessor, 1.0, true, true, false);
        };
    };
    //Jab 3
    if motion_kind == hash40("attack_13")
    && MotionModule::is_end(module_accessor) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    };
    //Dash Attack Speed
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_DASH);
        if (1.0..5.0).contains(&frame)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 4.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (1.0..5.0).contains(&frame)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){
            macros::SET_SPEED_EX(fighter, 0.15, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (6.0..11.0).contains(&frame)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 3.25, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (6.0..11.0).contains(&frame)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (11.0..=20.0).contains(&frame)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){
            macros::SET_SPEED_EX(fighter, 1.05, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (11.0..=20.0).contains(&frame)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if frame >= 21.0
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 0.45, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if frame >= 21.0
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    //Up Smash Jump Cancel
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && frame >= 30.0
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) != true
    && parried == 0 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    }
    //Fair
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if motion_kind == hash40("attack_air_f")
        && AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL) {
            FAIR_HIT[entry_id] = true;
        };
        if FAIR_HIT[entry_id] == true {
            MotionModule::change_motion(module_accessor, Hash40::new("attack_air_f_hit"), 1.0, 1.0, false, 0.0, false, false);
        };
        if motion_kind == hash40("attack_air_f_hit") {
            FAIR_HIT[entry_id] = false;
        };
    }
    else {
        FAIR_HIT[entry_id] = false;
    };
    //Homing Attack
    if [*FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL].contains(&status_kind) {
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
    }
    if motion_kind == hash40("special_n_homing")
    && AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL) {
        HOMING_ATTACK_HIT[entry_id] = true;
    };
    if HOMING_ATTACK_HIT[entry_id] {
        if [hash40("special_n_hit"), hash40("special_n_hit_1"), hash40("special_n_hit_2"), hash40("special_n_hit_3")].contains(&motion_kind) {
            HOMING_ATTACK_HIT[entry_id] = false;
        };
        MotionModule::change_motion(module_accessor, sonic_new_animation_hash, 1.0, 1.0, false, 0.0, false, false);
    };
    //Boost
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if fighter.global_table[SITUATION_KIND].get_i32() != SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        };
        if motion_kind == hash40("special_s_start") {
            if frame > 10.0 {
                if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(module_accessor);
                }
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
                if ControlModule::get_stick_y(module_accessor) <= -0.7 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
                };
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_DASH);
            }
            if SONIC_BOOST_SPEED[entry_id] < 2.0 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 4.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 3.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, 1.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if SONIC_BOOST_SPEED[entry_id] > 2.0
            && SONIC_BOOST_SPEED[entry_id] < 2.55 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*2.3, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*1.725, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*1.15, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*0.575, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if SONIC_BOOST_SPEED[entry_id] >= 2.55 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 6.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 4.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 3.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, 1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                if ControlModule::get_stick_x(module_accessor) >= 0.3 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN, true);
                }
                else if ControlModule::get_stick_x(module_accessor) <= -0.3 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN_RUN, true);
                }
                else {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, true);
                };
            };
        }
        if motion_kind == hash40("special_air_s_start") {
            if frame > 10.0 {
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                fighter.sub_transition_group_check_air_cliff();
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            if SONIC_BOOST_SPEED[entry_id] < 2.0 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 4.0, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 3.0, -1.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 2.0, -1.65, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, 1.0, -2.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if SONIC_BOOST_SPEED[entry_id] > 2.0
            && SONIC_BOOST_SPEED[entry_id] < 2.55 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*2.3, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*1.725, -1.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*1.15, -1.65, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, SONIC_BOOST_SPEED[entry_id]*0.575, -2.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if SONIC_BOOST_SPEED[entry_id] >= 2.55 {
                if (11.0..=19.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 6.0, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (20.0..=30.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 4.5, -1.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if (31.0..=44.0).contains(&frame) {
                    macros::SET_SPEED_EX(fighter, 3.0, -1.65, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if frame >= 45.0 {
                    macros::SET_SPEED_EX(fighter, 1.5, -2.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            };
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, true);
            };
        }
    };
    //Boost Effect
    SONIC_BOOST_GFX_COUNTER[entry_id] += 1;
    if SONIC_BOOST_GFX_COUNTER[entry_id] > 25 {
        SONIC_BOOST_GFX_COUNTER[entry_id] = 0;
    };
    if SONIC_BOOST[entry_id] >= 25.0 {
        if SONIC_BOOST_GFX_COUNTER[entry_id] == 10 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
        }
        if SONIC_BOOST_GFX_COUNTER[entry_id] >= 20 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
        }
    }
    else {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    };
    //Up Special
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {
        fighter.sub_transition_group_check_air_cliff();
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    };
    //Down Special
    //Spin Dash Canceled Dash Attack
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH
    && ControlModule::get_stick_y(module_accessor) <= -0.7
    && fighter.global_table[SITUATION_KIND].get_i32() != SITUATION_KIND_AIR {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        //Spin Dash Nerf
        if [hash40("special_s_spin"), hash40("special_s_dash_hi"), hash40("special_s_dash_lw")].contains(&motion_kind)
        && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_s_end_start"), 1.0, 1.0, 0.0, false, false);
        }
        //Bounce Bracelet
        if motion_kind == hash40("special_air_lw_start") {
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            if (12.0..=30.0).contains(&frame) {
                BOUNCE_BRACELET_POWER[entry_id] += 0.2778;
            }
            AttackModule::set_power_up(fighter.module_accessor, BOUNCE_BRACELET_POWER[entry_id]);
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
            && AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_lw_end"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        BOUNCE_BRACELET_POWER[entry_id] = 0.0;
        AttackModule::set_power_up(fighter.module_accessor, 1.0);
    }
}

pub fn install() {
    Agent::new("sonic")
    .on_line(Main, sonic_frame)
    .install()
    ;
}