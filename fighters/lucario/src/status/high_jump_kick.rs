use super::*;

unsafe extern "C" fn lucario_high_jump_kick_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S_COMMAND | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    SET_SPEED_EX(fighter, 6.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("high_jump_kick"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_high_jump_kick_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_high_jump_kick_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("high_jump_kick"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_high_jump_kick_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("lucario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_pre_status)
    .status(Init, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_init_status)
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_main_status)
    .status(Exec, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_exec_status)
    .status(End, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_end_status)
    .status(Exit, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK, lucario_high_jump_kick_exit_status)
    .install()
    ;
}