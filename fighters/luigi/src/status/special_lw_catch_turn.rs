use super::*;

unsafe extern "C" fn luigi_special_lw_catch_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_TURN, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_turn_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        PostureModule::set_scale(capture_boma, 0.001, false);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    ArticleModule::change_status(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_TURN, ArticleOperationTarget(0));
    ItemModule::set_have_item_visibility(boma, false, 0);
    MotionModule::change_motion(boma, Hash40::new("special_lw_catch_turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_catch_turn_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_catch_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    handle_mash(fighter);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        REVERSE_LR(fighter);
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_turn_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_turn_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_special_lw_end(fighter);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_turn_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_special_lw_exit(fighter);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, luigi_special_lw_catch_turn_exit_status)
    .install()
    ;
}