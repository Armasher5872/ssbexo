use super::*;

unsafe extern "C" fn littlemac_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_AttackS4(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    fighter.status_AttackS4_Main()
}

unsafe extern "C" fn littlemac_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, littlemac_attack_s4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, littlemac_attack_s4_end_status)
    .install()
    ;
}