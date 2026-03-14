use super::*;

unsafe extern "C" fn armstrong_attack_hi3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    fighter.status_AttackHi3_Common(mot.into(), mot.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_attack_hi3_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    armstrong_charge_move(fighter, 3.0, 10.0, 0.045, 7.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), true);
    fighter.status_AttackHi3_Main()
}

unsafe extern "C" fn armstrong_attack_hi3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    armstrong_clear_charge(fighter.module_accessor);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

unsafe extern "C" fn armstrong_attack_hi3_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    armstrong_clear_charge(fighter.module_accessor);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, armstrong_attack_hi3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI3, armstrong_attack_hi3_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_HI3, armstrong_attack_hi3_exit_status)
    .install()
    ;
}