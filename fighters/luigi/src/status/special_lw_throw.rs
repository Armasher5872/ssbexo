use super::*;

unsafe extern "C" fn luigi_special_lw_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_THROWN, false);
    }
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let direction = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_THROW, ArticleOperationTarget(0));
    luigi_special_lw_throw_sub_status(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW) {
        if direction == 2 {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw_hi"), L2CValue::Hash40s("special_air_lw_throw_hi"), false.into());
            grabbed_anim_selector(fighter, "special_lw_thrown_hi", 0.0, 1.0);
        }
        else if direction == 1 {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw_b"), L2CValue::Hash40s("special_air_lw_throw_b"), false.into());
            grabbed_anim_selector(fighter, "special_lw_thrown_b", 0.0, 1.0);
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw_f"), L2CValue::Hash40s("special_air_lw_throw_f"), false.into());
            grabbed_anim_selector(fighter, "special_lw_thrown_f", 0.0, 1.0);
        }
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw_plunger"), L2CValue::Hash40s("special_air_lw_throw_plunger"), false.into());
        grabbed_anim_selector(fighter, "special_lw_thrown_f", 0.0, 1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_throw_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_throw_sub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.ThrowUniq();
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_ThrowUniq as *const () as _));
    0.into()
}

unsafe extern "C" fn luigi_special_lw_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let direction = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let plunger_throw = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
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
            if !plunger_throw {
                if direction == 2 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw_hi"), -1.0, 1.0, 0.0, false, false);
                }
                else if direction == 1 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw_b"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw_f"), -1.0, 1.0, 0.0, false, false);
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw_plunger"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            if !plunger_throw {
                if direction == 2 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw_hi"), -1.0, 1.0, 0.0, false, false);
                }
                else if direction == 1 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw_b"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw_f"), -1.0, 1.0, 0.0, false, false);
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw_plunger"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        if !plunger_throw {
            PostureModule::set_scale(capture_boma, 0.001, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW) {
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            PostureModule::set_scale(capture_boma, 1.0, false);
            AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
        }
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
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

unsafe extern "C" fn luigi_special_lw_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    fighter.status_end_Throw()
}

unsafe extern "C" fn luigi_special_lw_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    fighter.sub_throw_uniq_process_exit()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW, luigi_special_lw_throw_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW, luigi_special_lw_throw_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW, luigi_special_lw_throw_main_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW, luigi_special_lw_throw_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW, luigi_special_lw_throw_exit_status)
    .install()
    ;
}