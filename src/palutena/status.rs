use super::*;

unsafe extern "C" fn palutena_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat3 = fighter.global_table[CMD_CAT3].get_i32();
    if cmd_cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH != 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_S_FLAG_SMASH);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_S_FLAG_SMASH);
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn palutena_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("palutena")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_special_s_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, palutena_special_hi_pre_status)
    .install()
    ;
}