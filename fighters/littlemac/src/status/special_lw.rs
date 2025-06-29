use super::*;

unsafe extern "C" fn littlemac_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let special_lw_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_motion_rate"));
    let ko_gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::clear_speed_all(fighter.module_accessor);
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD) {
        MotionModule::set_rate(fighter.module_accessor, special_lw_motion_rate);
    }
    else {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD) {
            ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_LITTLEMAC_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK);
        }
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD) {
            ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_LITTLEMAC_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK);
        }
    }
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        if ko_gauge == 100.0 {
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
        }
        WorkModule::sub_float(fighter.module_accessor, 1.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, littlemac_special_lw_main_status)
    .install()
    ;
}