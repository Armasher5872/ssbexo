use super::*;

unsafe extern "C" fn eflame_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK)(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn eflame_attack_s3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S3)(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn eflame_attack_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH)(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn eflame_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn eflame_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

unsafe extern "C" fn eflame_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SPRING) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("eflame")
    .status(End, *FIGHTER_STATUS_KIND_ATTACK, eflame_attack_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S3, eflame_attack_s3_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, eflame_attack_dash_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, eflame_attack_s4_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, eflame_attack_air_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, eflame_special_lw_pre_status)
    .install()
    ;
}