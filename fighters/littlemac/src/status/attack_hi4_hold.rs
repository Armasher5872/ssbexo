use super::*;

unsafe extern "C" fn littlemac_attack_hi4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    littlemac_status_attack_hi4_hold_common(fighter, hash40("attack_hi4_hold").into())
}

unsafe extern "C" fn littlemac_status_attack_hi4_hold_common(fighter: &mut L2CFighterCommon, motion_kind: L2CValue) -> L2CValue {
    let hold_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("attack_4_hold_frame"), 0);
    let hi4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_hi4_hold_frame"), 0);
    let keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_hi4_hold_keep_frame"), 0);
    let ratio = hi4_hold_frame as f32/hold_frame;
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    MotionModule::change_motion(fighter.module_accessor, motion_kind.get_hash(), 0.0, hi4_hold_frame as f32/ratio, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_attack_hi4_hold_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_attack_hi4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    fighter.status_AttackHi4Hold_main()
}

unsafe extern "C" fn littlemac_attack_hi4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    attack_4_hold(fighter);
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, littlemac_attack_hi4_hold_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, littlemac_attack_hi4_hold_end_status)
    .install()
    ;
}