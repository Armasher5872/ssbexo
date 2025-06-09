use super::*;

unsafe extern "C" fn cloud_attack_lw4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_attacklw4hold_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackLw4Hold_main as *const () as _))
}

unsafe extern "C" fn cloud_status_attacklw4hold_common(fighter: &mut L2CFighterCommon) {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_lw4_hold"} else {"attack_lw4_hold"};
    let attack_4_hold_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("attack_4_hold_frame"), 0);
    let attack_lw4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_frame"), 0);
    let attack_lw4_hold_keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_keep_frame"), 0);
    let loop_base_frame = (attack_lw4_hold_frame as f32)/attack_4_hold_frame;
    let motion_rate = (attack_lw4_hold_frame as f32)/loop_base_frame;
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    WorkModule::set_int(fighter.module_accessor, loop_base_frame as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, loop_base_frame as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, loop_base_frame as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, attack_lw4_hold_keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, motion_rate, false, 0.0, false, false);
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, cloud_attack_lw4_hold_main_status)
    .install()
    ;
}