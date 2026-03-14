use super::*;

unsafe extern "C" fn luigi_special_lw_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_pull_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if prev_status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_PULL, ArticleOperationTarget(0));
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_catch"), 0.0, 1.0, false, 0.0, false, false);
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_catchcommon"), -1);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let plunger_throw = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW) {
        if stick_y > 0.6 {
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        }
        else if stick_x < -0.6 {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        if !plunger_throw {
            if current_frame == 5.0 {
                PostureModule::set_scale(capture_boma, 0.8, false);
            }
            if current_frame == 6.0 {
                PostureModule::set_scale(capture_boma, 0.4, false);
            }
            if current_frame == 7.0 {
                PostureModule::set_scale(capture_boma, 0.1, false);
            }
            if current_frame >= 8.0 {
                PostureModule::set_scale(capture_boma, 0.001, false);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_pull_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if status_kind != *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW {
        if CatchModule::is_catch(fighter.module_accessor) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(fighter.module_accessor);
                PostureModule::set_scale(capture_boma, 1.0, false);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(fighter.module_accessor, true);
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_pull_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if status_kind != *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW {
        if CatchModule::is_catch(fighter.module_accessor) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(fighter.module_accessor);
                PostureModule::set_scale(capture_boma, 1.0, false);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(fighter.module_accessor, true);
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_special_lw_catch_pull_exit_status)
    .install()
    ;
}