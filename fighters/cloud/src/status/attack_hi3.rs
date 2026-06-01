use super::*;

unsafe extern "C" fn cloud_attack_hi3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    fighter.clear_lua_stack();
    let mut mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        mot = Hash40::new("punish_attack_hi3_1");
    }
    fighter.status_AttackHi3_Common(mot.into(), mot.into());
    if !StopModule::is_stop(boma) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_attack_hi3_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let count = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let motion_kind = MotionModule::motion_kind(boma);
    fighter.status_AttackHi3_Main();
    if StatusModule::is_changing(boma) {
        return 0.into();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if !StatusModule::is_changing(boma) {
                WorkModule::inc_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
            }
        }
    }
    if count == 1 && motion_kind == hash40("punish_attack_hi3_1") {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(boma, Hash40::new("punish_attack_hi3_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn cloud_attack_hi3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, cloud_attack_hi3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI3, cloud_attack_hi3_end_status)
    .install()
    ;
}