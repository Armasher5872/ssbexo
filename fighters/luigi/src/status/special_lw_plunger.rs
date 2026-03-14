use super::*;

unsafe extern "C" fn luigi_special_lw_plunger_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_plunger_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_plunger_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_PLUNGER, ArticleOperationTarget(0));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_plunger"), L2CValue::Hash40s("special_air_lw_plunger"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_plunger_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_plunger_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_plunger"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_plunger"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        let plunger_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER);
        let plunger_status_kind = StatusModule::status_kind(plunger_boma);
        let collision_pos1_x = WorkModule::get_float(plunger_boma, 0x19);
        let collision_pos1_y = WorkModule::get_float(plunger_boma, 0x1A);
        let collision_pos1_z = WorkModule::get_float(plunger_boma, 0x1B);
        if plunger_status_kind == *WEAPON_LUIGI_PLUNGER_STATUS_KIND_SHOOT {
            if MotionModule::frame(plunger_boma) > 1.0 {
                ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: collision_pos1_x, y: collision_pos1_y, z: collision_pos1_z}, true, false);
            }
        }
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

unsafe extern "C" fn luigi_special_lw_plunger_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if status_kind != *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL {
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    delete_plunger(global_fighter, false);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER, luigi_special_lw_plunger_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER, luigi_special_lw_plunger_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER, luigi_special_lw_plunger_main_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER, luigi_special_lw_plunger_end_status)
    .install()
    ;
}