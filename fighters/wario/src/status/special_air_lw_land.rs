use super::*;

//Down Special Land Pre Status
unsafe extern "C" fn wario_special_lw_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Down Special Land Init Status
unsafe extern "C" fn wario_special_lw_land_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Land Main Status
unsafe extern "C" fn wario_special_lw_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_END {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_heavy"), 0.0, 31.0/28.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_land"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_lw_land_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_lw_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Down Special Land Exec Status
unsafe extern "C" fn wario_special_lw_land_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Land End Status
unsafe extern "C" fn wario_special_lw_land_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    0.into()
}

//Down Special Land Exit Status
unsafe extern "C" fn wario_special_lw_land_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND, wario_special_lw_land_exit_status)
    .install()
    ;
}