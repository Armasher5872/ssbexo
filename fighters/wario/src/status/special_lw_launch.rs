use super::*;

unsafe extern "C" fn wario_special_lw_launch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn wario_special_lw_launch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, true);
    0.into()
}

unsafe extern "C" fn wario_special_lw_launch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_launch"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_lw_launch_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_lw_launch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            if current_frame > 10.0 {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP) {
        let special_lw_charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        ADD_SPEED_NO_LIMIT(fighter, 0, 1.0+special_lw_charge);
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn wario_special_lw_launch_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_special_lw_launch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    0.into()
}

unsafe extern "C" fn wario_special_lw_launch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LAUNCH, wario_special_lw_launch_exit_status)
    .install()
    ;
}