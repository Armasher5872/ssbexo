use super::*;

pub unsafe extern "C" fn miiswordsman_special_n3_slash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0 as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n3_slash_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n3_slash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n32"), L2CValue::Hash40s("special_air_n32"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_n3_slash_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_n3_slash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n32"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n32"), -1.0, 1.0, 0.0, false, false);
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && frame > 3.0 {
        if situation_kind == *SITUATION_KIND_AIR {
            if fighter.is_cat_flag(Cat1::AttackAirN)
            || fighter.is_cat_flag(Cat1::AttackAirF)
            || fighter.is_cat_flag(Cat1::AttackAirB)
            || fighter.is_cat_flag(Cat1::AttackAirHi)
            || fighter.is_cat_flag(Cat1::AttackAirLw)
            || fighter.is_cat_flag(Cat1::AttackN)
            || fighter.is_cat_flag(Cat1::AttackS3)
            || fighter.is_cat_flag(Cat1::AttackHi3)
            || fighter.is_cat_flag(Cat1::AttackLw3)
            || fighter.is_cat_flag(Cat1::AttackS4)
            || fighter.is_cat_flag(Cat1::AttackHi4)
            || fighter.is_cat_flag(Cat1::AttackLw4) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
            }
        }
        else {
            if fighter.is_cat_flag(Cat1::AttackN) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            }
            if (fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN)) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), false.into());
            }
            if fighter.is_cat_flag(Cat1::Catch) {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            }
            if (fighter.is_cat_flag(Cat1::Catch) && fighter.is_cat_flag(Cat1::Dash)) {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
        if fighter.is_cat_flag(Cat1::SpecialHi) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
        }
        if fighter.is_cat_flag(Cat1::SpecialLw) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
        if fighter.is_cat_flag(Cat1::SpecialS) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
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

pub unsafe extern "C" fn miiswordsman_special_n3_slash_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n3_slash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
    WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n3_slash_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
    WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    0.into()
}