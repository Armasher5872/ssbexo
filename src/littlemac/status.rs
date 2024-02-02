use super::*;

/*   NEUTRAL SPECIAL STATUS SCRIPTS   */

unsafe extern "C" fn littlemac_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2.into(), false.into());
    0.into()
}

unsafe extern "C" fn littlemac_special_n2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, littlemac_special_n_main_status)
    .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, littlemac_special_n2_pre_status)
    .install()
    ;
}