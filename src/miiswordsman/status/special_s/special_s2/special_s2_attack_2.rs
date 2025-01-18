use super::*;

pub unsafe extern "C" fn miiswordsman_special_s2_attack_2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s2_attack_2_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*lr;
    match stick_y {
        _ if stick_y >= 0.25 => {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
        _ if stick_y <= -0.25 => {
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
        _ => {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
    }
    if stick_x <= -0.25 {
        WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s2_attack_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rending_serrations_direction = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    match rending_serrations_direction {
        0 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s2_3s"), L2CValue::Hash40s("special_air_s2_3s"), false.into());
        }
        1 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s2_3hi"), L2CValue::Hash40s("special_air_s2_3hi"), false.into());
        }
        2 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s2_3lw"), L2CValue::Hash40s("special_air_s2_3lw"), false.into());
        }
        3 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s2_3b"), L2CValue::Hash40s("special_air_s2_3b"), false.into());
        }
        _ => {}
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_s2_attack_2_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_s2_attack_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let rending_serrations_direction = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s2_3hi") {
        if frame == 11.0 {
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
        if (12.0..36.0).contains(&frame) {
            if situation_kind == *SITUATION_KIND_GROUND
            && prev_situation_kind == *SITUATION_KIND_AIR {
                GroundModule::set_attach_ground(fighter.module_accessor, true);
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        if frame == 37.0 {
            GroundModule::set_attach_ground(fighter.module_accessor, true);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        match rending_serrations_direction {
            0 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_3s"), -1.0, 1.0, 0.0, false, false);
            }
            1 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_3hi"), -1.0, 1.0, 0.0, false, false);
            }
            2 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_3lw"), -1.0, 1.0, 0.0, false, false);
            }
            3 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_3b"), -1.0, 1.0, 0.0, false, false);
            }
            _ => {}
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        match rending_serrations_direction {
            0 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_3s"), -1.0, 1.0, 0.0, false, false);
            }
            1 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_3hi"), -1.0, 1.0, 0.0, false, false);
            }
            2 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_3lw"), -1.0, 1.0, 0.0, false, false);
            }
            3 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_3b"), -1.0, 1.0, 0.0, false, false);
            }
            _ => {}
        }
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s2_attack_2_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    0.into()
}