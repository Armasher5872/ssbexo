use super::*;

//Side Special Exec Status
unsafe extern "C" fn mario_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if (12.0..24.0).contains(&frame) {
            macros::SET_SPEED_EX(fighter, 1.5, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (25.0..40.0).contains(&frame) {
            macros::SET_SPEED_EX(fighter, 0.75, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

//Up Special Main Status, goes into the loop status
unsafe extern "C" fn mario_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_loop as *const () as _));
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
}

//Up Special Loop, goes into the Super Jump Punch Main function
unsafe extern "C" fn special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_main();
    0.into()
}

//Down Special Pre Status
unsafe extern "C" fn mario_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Down Special Shoot Pre Status
unsafe extern "C" fn mario_special_lw_shoot_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exec_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, mario_special_hi_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_pre_status)
    .status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_pre_status)
    .install()
    ;
}