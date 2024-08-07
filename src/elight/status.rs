use super::*;

unsafe extern "C" fn elight_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK)(fighter);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn elight_attack_s3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S3)(fighter);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn elight_attack_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH)(fighter);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn elight_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn elight_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn elight_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn elight_special_s_forward_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn elight_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SPRING) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("elight")
    .status(End, *FIGHTER_STATUS_KIND_ATTACK, elight_attack_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S3, elight_attack_s3_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, elight_attack_dash_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, elight_attack_s4_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, elight_attack_air_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, elight_special_s_pre_status)
    .status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, elight_special_s_forward_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, elight_special_lw_pre_status)
    .install()
    ;
}