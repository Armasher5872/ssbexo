use super::*;

//Fujin Uraken Pre Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW3 as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

//Fujin Uraken Init Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Fujin Uraken Main Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("attack_lw3_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_lw3_cancel_attack_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_lw3_cancel_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

//Fujin Uraken Exec Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Fujin Uraken End Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Fujin Uraken Exit Status
unsafe extern "C" fn demon_attack_lw3_cancel_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_pre_status)
    .status(Init, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_init_status)
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_main_status)
    .status(Exec, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_exec_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK, demon_attack_lw3_cancel_attack_exit_status)
    .install()
    ;
}