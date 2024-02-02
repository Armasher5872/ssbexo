use super::*;

pub unsafe extern "C" fn miiswordsman_special_hi3_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi3_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hi3_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi3_landing_frame") as u64);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_END);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_WORK_HOLD_FRAME);
    WorkModule::set_int(fighter.module_accessor, hi3_landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi3"), L2CValue::Hash40s("special_air_hi3"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_hi3_start_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_hi3_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let hi3_charge_spd_div = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi3_charge_spd_div"));
    let is_end = MotionModule::is_end(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, hi3_charge_spd_div);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_END);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi3"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_END);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE));
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*FIGHTER_KINETIC_TYPE_MIISWORDSMAN_SPECIAL_AIR_HI3));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi3"), -1.0, 1.0, 0.0, false, false);
    }
    if is_end {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD.into(), true.into());
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_TRANSITION_TERM_ID_END) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi3_start_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}