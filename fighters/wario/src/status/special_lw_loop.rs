use super::*;

unsafe extern "C" fn wario_special_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn wario_special_lw_loop_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn wario_special_lw_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::add_float(fighter.module_accessor, 1.0/30.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    0.into()
}

unsafe extern "C" fn wario_special_lw_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    }
    0.into()
}

unsafe extern "C" fn wario_special_lw_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LOOP, wario_special_lw_loop_exit_status)
    .install()
    ;
}