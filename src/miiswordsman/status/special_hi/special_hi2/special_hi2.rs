use super::*;

pub unsafe extern "C" fn miiswordsman_special_hi2_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi2_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_SDUSH_STATUS_WORK_ID_FLAG_CONTINUE);
    let hi2_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_stop_y_frame"));
    WorkModule::set_int(fighter.module_accessor, hi2_stop_y_frame, *FIGHTER_MIISWORDSMAN_SDUSH_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_SDUSH_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001fe40 as *const () as _));
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08 - 1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi2_hold"), L2CValue::Hash40s("special_hi2_hold_air"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_hi2_start_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710001fe40(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_SDUSH_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    0.into()
}

unsafe extern "C" fn miiswordsman_special_hi2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi2_hold"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi2_hold_air"), -1.0, 1.0, 0.0, false, false);
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        miiswordsman_special_hi2_handle_bound(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn miiswordsman_special_hi2_handle_bound(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_SDUSH_STATUS_WORK_ID_FLAG_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let hi2_crush_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_crush_angle"));
        let rad = (hi2_crush_angle + 90.0).to_radians();
        if rad < angle {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi2_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    miiswordsman_special_hi2_set_function(fighter);
    fighter.super_jump_punch_end(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_reset_common_condition as *const () as _));
    0.into()
}

unsafe extern "C" fn miiswordsman_special_hi2_set_function(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hi2_fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_fall_x_mul"));
    let hi2_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_landing_frame"));
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND].contains(&status_kind_interrupt)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::set_float(fighter.module_accessor, hi2_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::set_float(fighter.module_accessor, hi2_fall_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    0.into()
}

//Statuses that need to be translated
/* 

//Special Hi 2 Init

//Special Hi 2 Exec

*/