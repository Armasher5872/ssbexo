use super::*;

pub unsafe extern "C" fn miifighter_special_lw1_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw1_attack"), L2CValue::Hash40s("special_air_lw1_attack"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw1_attack_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw1_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_lw1_attack"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_lw1_attack"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_float(boma, 12.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 10.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    WorkModule::set_int(boma, 0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_float(boma, 12.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 10.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    WorkModule::set_int(boma, 0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    0.into()
}