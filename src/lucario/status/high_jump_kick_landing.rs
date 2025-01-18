use super::*;

unsafe extern "C" fn lucario_high_jump_kick_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S_COMMAND | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_landing_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.5, 0.0);
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("high_jump_kick_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_high_jump_kick_landing_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_high_jump_kick_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_landing_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_landing_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_landing_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("lucario")
    .status(Pre, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_pre_status)
    .status(Init, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_init_status)
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_main_status)
    .status(Exec, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_exec_status)
    .status(End, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_end_status)
    .status(Exit, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, lucario_high_jump_kick_landing_exit_status)
    .install()
    ;
}