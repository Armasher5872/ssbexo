use super::*;

unsafe extern "C" fn fox_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn fox_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn fox_special_lw_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    //If you've hit the move and haven't hit shield, transition to jump cancel
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        fighter.check_jump_cancel(false, false);
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 4.0 {
        //If you can, transition into the appropriate status kind
        fighter.check_jump_cancel(false, false);
    }
    original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

unsafe extern "C" fn fox_special_lw_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Checks if the current frame in the status is greater than or equal to 4.0 (Effectively 8 frames into Shine)
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 4.0 {
        //If you can, transition into the appropriate status kind
        fighter.check_jump_cancel(false, false);
    }
    original_status(Exec, fighter, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP)(fighter)
}

pub fn install() {
    Agent::new("fox")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, fox_special_s_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, fox_special_hi_pre_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, fox_special_lw_exec_status)
    .status(Exec, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, fox_special_lw_loop_exec_status)
    .install()
    ;
}