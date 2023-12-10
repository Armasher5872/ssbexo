use super::*;

unsafe extern "C" fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let point_offset_x = PostureModule::lr(boma) * (FUTURESIGHT_X[entry_id] - PostureModule::pos_x(boma));
    let point_offset_y = FUTURESIGHT_Y[entry_id] - PostureModule::pos_y(boma);
    //Stored Power Addition
    if frame < 2.0 { 
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    };
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD)
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && STORED_POWER_ENABLED[entry_id] == 0 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        STORED_POWER_POINT[entry_id] += 7;
    };
    if STORED_POWER_POINT[entry_id] >= 100 {
        STORED_POWER_POINT[entry_id] = 100;
    }
    if STORED_POWER_POINT[entry_id] <= 0 {
        STORED_POWER_ENABLED[entry_id] = 0;
    }
    //Dodge Cancels
    if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
        if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
        }
    }
    //Ghost Dash
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if (6.0..=8.0).contains(&frame) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) == true {
                GHOST_DASH_ENABLED[entry_id] = true;
            }
        }
        if (9.0..21.0).contains(&frame) {
            if GHOST_DASH_ENABLED[entry_id] == true {
                ModelModule::set_visibility(boma, false);
            }
        }
        if frame > 21.0 {
            GHOST_DASH_ENABLED[entry_id] = false;
        }
    }
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH {
        GHOST_DASH_ENABLED[entry_id] = false;
    }
    if GHOST_DASH_ENABLED[entry_id] == false {
        ModelModule::set_visibility(boma, true);
    }
    //Shield Special
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
        MotionModule::change_motion(boma, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
    }
    if motion_kind == hash40("special_shield") {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        STORED_POWER_ENABLED[entry_id] = 1;
    };
    STORED_POWER_TIMER[entry_id] -= 1;
    if STORED_POWER_ENABLED[entry_id] == 1 {
        //GFX
        STORED_POWER_GFX_TIMER[entry_id] += 1;
        if STORED_POWER_GFX_TIMER[entry_id] == 10 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
        }
        if STORED_POWER_GFX_TIMER[entry_id] >= 20 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            STORED_POWER_GFX_TIMER[entry_id] = 0;
        }
        //Stored Power Decrement
        if STORED_POWER_TIMER[entry_id] <= 0 {
            STORED_POWER_POINT[entry_id] -= 2;
            STORED_POWER_TIMER[entry_id] = 60;
        }
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
            STORED_POWER_POINT[entry_id] -= 10;
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        }
        //Neutral Special
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                if WorkModule::get_int(boma, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
                }
                if MotionModule::is_end(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                }
            }
        }
        //Future Sight
        if HAS_FUTURESIGHT[entry_id] {
            FUTURESIGHT_CURRENT_FRAME[entry_id] += 1;
            if FUTURESIGHT_CURRENT_FRAME[entry_id] <= FUTURESIGHT_FUSE_TIME {
                if FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] <= 300
                && FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] > 298 {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_metamon_aura"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_flash"), false, false);
                    macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                }
                else if FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] < 298 && FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] > 31 && FUTURESIGHT_CURRENT_FRAME[entry_id] % 60 == 0 {
                    macros::EFFECT(fighter, Hash40::new("sys_metamon_aura"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                }
                else if FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] == 30 {
                    macros::EFFECT(fighter, Hash40::new("sys_explosion_sign"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.85, 5, 5, 0, 0, 0, 0, false);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
                }
                else if FUTURESIGHT_FUSE_TIME - FUTURESIGHT_CURRENT_FRAME[entry_id] == 5 {
                    macros::EFFECT(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.85, 5, 5, 0, 0, 0, 0, false);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
                }
                FUTURESIGHT_LAST_STATUS[entry_id] = status_kind;
            }
            else if FUTURESIGHT_CURRENT_FRAME[entry_id] <= FUTURESIGHT_FUSE_TIME + FUTURESIGHT_EXPLOSION_TIME {
                if status_kind != FUTURESIGHT_LAST_STATUS[entry_id] || [*FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
                    FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] = FUTURESIGHT_HIT_COOLDOWN_TIME;
                }
                if FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] > 0 {
                    FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] -= 1;
                }
                else {
                    macros::ATTACK(fighter, 7, 1, Hash40::new("top"), 12.0, 150, 60, 0, 85, 15.0, 0.0, point_offset_y, point_offset_x, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
                }
                if (FUTURESIGHT_CURRENT_FRAME[entry_id] - FUTURESIGHT_FUSE_TIME) % 3 == 0 {
                    macros::EFFECT(fighter, Hash40::new("sys_hit_purple"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
                    macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
                }
                if (FUTURESIGHT_CURRENT_FRAME[entry_id] - FUTURESIGHT_FUSE_TIME) % 20 == 0 {
                    macros::EFFECT(fighter, Hash40::new("sys_dead_dark"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
                }
                FUTURESIGHT_LAST_STATUS[entry_id] = status_kind;
            }
            else {
                FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
                HAS_FUTURESIGHT[entry_id] = false;
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_metamon_aura"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_flash"), false, false);
                AttackModule::clear(boma, 7, false);
            }
        }
        //Up Special
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        && frame <= 1.0 {
            GROUNDED_TELEPORT[entry_id] = false;
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                GROUNDED_TELEPORT[entry_id] = true;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if STORED_POWER_ENABLED[entry_id] == 1
            && frame >= 4.0 {
                CancelModule::enable_cancel(boma);
            }
            if CancelModule::is_enable_cancel(boma) {
                fighter.sub_wait_ground_check_common(false.into());
                SPEED_ADD[entry_id] = true;
            }
        }
        if SPEED_ADD[entry_id] == true
        && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        && ![*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 7.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            STORED_POWER_POINT[entry_id] -= 20;
            SPEED_ADD[entry_id] = false;
        }
        //Up Special Facing Direction
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 && MotionModule::is_end(boma) {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                PostureModule::set_stick_lr(boma, 0.0);
                PostureModule::update_rot_y_lr(boma);
            }
        }
        //Actionable Teleport
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && frame > 8.0 {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                UP_SPECIAL_CANCEL[entry_id] = true;
                CancelModule::enable_cancel(boma);
                if GROUNDED_TELEPORT[entry_id] != true {
                    WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
        }
        //Up Special Double Jump Return
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            if frame <= 2.0 {
                UP_SPECIAL_JUMP_REFRESH[entry_id] = true;
            }
            else {
                UP_SPECIAL_JUMP_REFRESH[entry_id] = false;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_AERIAL) && UP_SPECIAL_JUMP_REFRESH[entry_id] {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
            UP_SPECIAL_JUMP_REFRESH[entry_id] = false;
        }
    }
    else {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_metamon_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_flash"), false, false);
        STORED_POWER_GFX_TIMER[entry_id] = 0;
    };
}

pub fn install() {
    Agent::new("mewtwo")
    .on_line(Main, mewtwo_frame)
    .install()
    ;
}