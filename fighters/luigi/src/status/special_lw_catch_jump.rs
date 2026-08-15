use super::*;

unsafe extern "C" fn luigi_special_lw_catch_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_jump_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        PostureModule::set_scale(capture_boma, 0.001, false);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    ControlModule::reset_flick_x(boma);
    ControlModule::reset_flick_sub_x(boma);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xFE));
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    ArticleModule::change_status(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_TURN, ArticleOperationTarget(0));
    ItemModule::set_have_item_visibility(boma, false, 0);
    MotionModule::change_motion(boma, Hash40::new("special_lw_catch_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_catch_jump_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_catch_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let is_attack = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let lr = PostureModule::lr(boma);
    let x_stick = stick_x*lr;
    let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
    let jump_neutral_y = WorkModule::get_param_float(boma, hash40("common"), hash40("jump_neutral_y"));
    handle_mash(fighter);
    if x_stick < -run_stick_x && is_attack {
        WorkModule::set_int(boma, 1, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW.into(), false.into());
    }
    if x_stick > run_stick_x && is_attack {
        WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW.into(), false.into());
    }
    if stick_y > jump_neutral_y && is_attack {
        WorkModule::set_int(boma, 2, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_jump_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_special_lw_end(fighter);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_jump_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_special_lw_exit(fighter);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, luigi_special_lw_catch_jump_exit_status)
    .install()
    ;
}