use super::*;

unsafe extern "C" fn bayonetta_attack_air_f_dash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32, 0);
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_dash"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_f_dash_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() 
    || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_dash"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if current_frame >= 23.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_dash_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_S4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_smash"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_f_smash_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() 
    || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_s4_s"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if current_frame > 30.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_f_smash_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_hi_smash"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_hi_smash_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() 
    || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_hi4"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if current_frame > 35.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_hi_smash_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_lw_smash"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_lw_smash_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() 
    || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_lw4"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if current_frame > 25.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_attack_air_lw_smash_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    }
    else {
        let mut status_kind = *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U;
        let enable_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_D_ENABLE_FRAME);
        if 0 < enable_frame {
            status_kind = *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D;
        }
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, status_kind);
        return 1.into()
    }
    0.into()
}

unsafe extern "C" fn bayonetta_special_air_s_u_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn bayonetta_special_air_s_d_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("bayonetta")
    .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_pre_status)
    .status(Init, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_init_status)
    .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_main_status)
    .status(Exec, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_exec_status)
    .status(End, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_end_status)
    .status(Exit, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, bayonetta_attack_air_f_dash_exit_status)
    .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_pre_status)
    .status(Init, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_init_status)
    .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_main_status)
    .status(Exec, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_exec_status)
    .status(End, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_end_status)
    .status(Exit, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, bayonetta_attack_air_f_smash_exit_status)
    .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_pre_status)
    .status(Init, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_init_status)
    .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_main_status)
    .status(Exec, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_exec_status)
    .status(End, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_end_status)
    .status(Exit, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, bayonetta_attack_air_hi_smash_exit_status)
    .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_pre_status)
    .status(Init, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_init_status)
    .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_main_status)
    .status(Exec, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_exec_status)
    .status(End, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_end_status)
    .status(Exit, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH, bayonetta_attack_air_lw_smash_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, bayonetta_special_s_pre_status)
    .status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, bayonetta_special_air_s_u_pre_status)
    .status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, bayonetta_special_air_s_d_pre_status)
    .install()
    ;
}