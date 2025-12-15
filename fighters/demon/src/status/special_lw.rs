use super::*;

unsafe extern "C" fn demon_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710002a3b0(fighter, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).into(), FIGHTER_STATUS_ATTR_START_TURN.into(), FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into())
}

unsafe extern "C" fn fun_710002a3b0(fighter: &mut L2CFighterCommon, log_mask_flag: L2CValue, status_attr: L2CValue, power_up_attack_bit: L2CValue) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let ground_correct_kind;
    if situation_kind != *SITUATION_KIND_GROUND {
        ground_correct_kind = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        ground_correct_kind = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK;
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, ground_correct_kind as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, log_mask_flag.get_u64(), status_attr.get_u32(), power_up_attack_bit.get_u32(), 0);
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, demon_special_lw_pre_status)
    .install()
    ;
}