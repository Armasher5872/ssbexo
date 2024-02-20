use super::*;

unsafe extern "C" fn luigi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
        0.into()
    }
    else {
        fighter.status_pre_Throw()
    }
}

unsafe extern "C" fn luigi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw"), L2CValue::Hash40s("special_air_lw_throw"), false.into());
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_f"), false, -1.0);
        fighter.sub_shift_status_main(L2CValue::Ptr(luigi_throw_main_loop as *const () as _))
    }
    else {
        fighter.status_Throw()
    }
}

unsafe extern "C" fn luigi_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
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

unsafe extern "C" fn luigi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.status_end_Throw();
        let mut remove_object_id: bool = true;
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_CUT {
            remove_object_id = false;
        }
        if remove_object_id {
            let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
            ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        }
        if LUIGI_CYCLONE_RNG[entry_id] > 1 {
            LUIGI_CYCLONE_RNG[entry_id] -= 2;
        }
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        0.into()
    }
    else {
        fighter.status_end_Throw()
    }
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, luigi_throw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, luigi_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, luigi_throw_end_status)
    .install()
    ;
}