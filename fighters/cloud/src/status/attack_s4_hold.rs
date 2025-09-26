use super::*;

unsafe extern "C" fn cloud_attack_s4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_s4_hold"} else {"attack_s4_hold"};
    let hold_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("attack_4_hold_frame"), 0);
    let s4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0);
    let keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_keep_frame"), 0);
    let ratio = s4_hold_frame as f32/hold_frame;
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, s4_hold_frame as f32/ratio, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_attack_s4_hold_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_attack_s4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4Hold_main()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, cloud_attack_s4_hold_main_status)
    .install()
    ;
}