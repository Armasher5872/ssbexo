use super::*;

unsafe extern "C" fn pzenigame_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let kinetic_type = KineticModule::get_kinetic_type(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let pos_x = PostureModule::pos_x(module_accessor);
    let pos_y = PostureModule::pos_y(module_accessor);
    let lr = PostureModule::lr(module_accessor);
    //Forward Smash Cancels
    if [hash40("attack_s4_s"), hash40("attack_s4_hi"), hash40("attack_s4_lw")].contains(&motion_kind)
    && (14.0..17.0).contains(&frame) {
        if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, true);
        }
    }
    //Shield Special
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
    }
    if motion_kind == hash40("special_shield") {
        WorkModule::set_flag(module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    };
    if motion_kind == hash40("special_shield") && frame == 50.0 {
        if RAIN_DANCE_ACTIVE[entry_id] == false || RAIN_DANCE_FRAME[entry_id] != 600.0 {
            RAIN_DANCE_ACTIVE[entry_id] = true;
            RAIN_DANCE_FRAME[entry_id] = 600.0;
            RAIN_DANCE_X1[entry_id] = pos_x - (18.0 * lr);
            RAIN_DANCE_X2[entry_id] = pos_x + (18.0 * lr);
            RAIN_DANCE_Y1[entry_id] = pos_y;
            RAIN_DANCE_Y2[entry_id] = pos_y + 20.0;
        }
    }
    //Squirtle's Rain Dance Region Detection
    let point_offset_x1 = lr * (RAIN_DANCE_X1[entry_id] - pos_x);
    let point_offset_x2 = lr * (RAIN_DANCE_X2[entry_id] - pos_x);
    let point_offset_y1 = RAIN_DANCE_Y1[entry_id] - pos_y;
    if (pos_x <= RAIN_DANCE_X2[entry_id] && pos_x >= RAIN_DANCE_X1[entry_id]) && (pos_y >= RAIN_DANCE_Y1[entry_id] && pos_y <= RAIN_DANCE_Y2[entry_id]) && IN_RAIN_DANCE[entry_id] != true {
        IN_RAIN_DANCE[entry_id] = true;
    }
    else {
        IN_RAIN_DANCE[entry_id] = false;
    }
    //Rain Dance Effects
    if RAIN_DANCE_ACTIVE[entry_id] == true {
        RAIN_DANCE_FRAME[entry_id] -= 1.0;
        if RAIN_DANCE_FRAME[entry_id] <= 600.0 && RAIN_DANCE_FRAME[entry_id] >= 598.0 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
        }
        else if RAIN_DANCE_FRAME[entry_id] < 598.0 && RAIN_DANCE_FRAME[entry_id] % 30.0 == 0.0 {
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1, point_offset_y1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2, point_offset_y1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1 - (12.0*lr), point_offset_y1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2 + (12.0*lr), point_offset_y1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
        if RAIN_DANCE_FRAME[entry_id] <= 0.0 {
            RAIN_DANCE_ACTIVE[entry_id] = false;
        }
    }
    if RAIN_DANCE_ACTIVE[entry_id] == false {
        RAIN_DANCE_FRAME[entry_id] = -1.0;
        RAIN_DANCE_X1[entry_id] = 0.0;
        RAIN_DANCE_X2[entry_id] = 0.0;
        RAIN_DANCE_Y1[entry_id] = 0.0;
        RAIN_DANCE_Y2[entry_id] = 0.0;
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
    }
    //Actionable Neutral Special
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
            if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                WorkModule::unable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    //Jump Cancelable Side Special
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if MotionModule::frame(module_accessor) >= 10.0
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                PZENIGAME_WITHDRAW_JUMP[entry_id] = 1;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
            else if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR
            && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                PZENIGAME_WITHDRAW_JUMP[entry_id] = 1;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
    }
    //Side Special Related Stuff
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind)
    && PZENIGAME_WITHDRAW_JUMP[entry_id] == 1 {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
        PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
    }
    if [*FIGHTER_KINETIC_TYPE_JUMP, *FIGHTER_KINETIC_TYPE_JUMP_ICE, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF_VERTICAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND].contains(&kinetic_type)
    && PZENIGAME_WITHDRAW_JUMP[entry_id] == 1 {
        PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
    }
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
        if (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT || StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP_AERIAL) && PZENIGAME_WITHDRAW_JUMP[entry_id] != 0 {
            PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
        }
        if PZENIGAME_WITHDRAW_JUMP[entry_id] == 0 {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
}

unsafe extern "C" fn pzenigame_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
}

pub fn install() {
    Agent::new("pzenigame")
    .on_start(pzenigame_init)
    .on_line(Main, pzenigame_frame)
    .install()
    ;
}