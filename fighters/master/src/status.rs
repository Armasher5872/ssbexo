use super::*;

unsafe extern "C" fn master_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut flick: bool = false;
    let cmd_cat3 = fighter.global_table[CMD_CAT3].get_i32();
    if cmd_cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH != 0 {
        flick = true;
    }
    if flick {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_FLICK);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_FLICK);
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn master_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut flag: i32 = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
    let mut int: i32 = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
    let mut float: i32 = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    let mut fs: i32 = 0;
    let mut log_mask_flag: u64 = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64;
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_TURN, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_LANDING_1].contains(&prev_status_kind) {
        flag = *FIGHTER_MASTER_STATUS_WORK_KEEP_FLAG_SPECIAL_LW_FLAG;
        int = *FIGHTER_MASTER_STATUS_WORK_KEEP_FLAG_SPECIAL_LW_INT;
        float = *FIGHTER_MASTER_STATUS_WORK_KEEP_FLAG_SPECIAL_LW_FLOAT;
        fs = *FS_SUCCEEDS_KEEP_NO_REACTION | *FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SLOPE;
        log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, flag, int, float, fs);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, log_mask_flag, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("master")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, master_special_s_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, master_special_lw_pre_status)
    .install()
    ;
}