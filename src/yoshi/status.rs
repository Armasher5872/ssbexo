use super::*;

unsafe extern "C" fn yoshi_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Script is effectively vanilla script, except the Armor Values were removed
    let turn_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    if fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor) > turn_cont_value {
        let turn_count = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"));
        WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    } 
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.status_JumpAerial();
    0.into()
}

unsafe extern "C" fn yoshi_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_START_TURN as u32 | *FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE as u32), *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn yoshi_special_n_1_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn yoshi_special_lw_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("yoshi")
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, yoshi_jump_aerial_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, yoshi_special_n_pre_status)
    .status(Pre, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, yoshi_special_n_1_pre_status)
    .status(Pre, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_LANDING, yoshi_special_lw_landing_pre_status)
    .install()
    ;
}