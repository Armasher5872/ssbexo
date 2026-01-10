use super::*;

//Down Special Attack Pre Status
unsafe extern "C" fn metaknight_special_lw_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Down Special Attack Init Status
unsafe extern "C" fn metaknight_special_lw_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER);
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

//Down Special Attack Main Status
unsafe extern "C" fn metaknight_special_lw_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("metaknight_sword"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
    WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Down Special Attack Exec Status
unsafe extern "C" fn metaknight_special_lw_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Attack End Status
unsafe extern "C" fn metaknight_special_lw_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT);
    0.into()
}

//Down Special Attack Exit Status
unsafe extern "C" fn metaknight_special_lw_attack_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT);
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_pre_status)
    .status(Init, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_init_status)
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_main_status)
    .status(Exec, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_exec_status)
    .status(End, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_end_status)
    .status(Exit, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, metaknight_special_lw_attack_exit_status)
    .install()
    ;
}