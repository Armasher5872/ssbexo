use super::*;

unsafe extern "C" fn purin_attack_lw4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn purin_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.status_AttackLw4_common();
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    attack_lw_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(purin_attack_lw4_main_loop as *const () as _))   
}

unsafe extern "C" fn attack_lw_set_kinetic(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}

unsafe extern "C" fn purin_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        let combo = ComboModule::count(boma) as i32;
        let lw4_combo_max = WorkModule::get_param_int(boma, hash40("lw4_combo_max"), 0);
        if combo < lw4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_lw4_mtrans_common(hash40("attack_lw4").into());
        }
        if StatusModule::is_situation_changed(boma) {
            attack_lw_set_kinetic(fighter);
        }
    }
    if CancelModule::is_enable_cancel(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_AIR
        && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::trans_move_speed(boma).y() < 0.0 && fighter.sub_transition_group_check_air_landing().get_bool() {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
            return 0.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("purin")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4, purin_attack_lw4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, purin_attack_lw4_main_status)
    .install()
    ;
}