use super::*;

unsafe extern "C" fn luigi_special_lw_catch_walk_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_walk_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        PostureModule::set_scale(capture_boma, 0.001, false);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_walk_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    ArticleModule::change_status(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_WALK, ArticleOperationTarget(0));
    ItemModule::set_have_item_visibility(boma, false, 0);
    MotionModule::change_motion(boma, Hash40::new("special_lw_catch_walk"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_catch_walk_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_catch_walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let is_attack = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let lr = PostureModule::lr(boma);
    let x_stick = stick_x*lr;
    let walk_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("walk_stick_x"));
    let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
    let jump_neutral_y = WorkModule::get_param_float(boma, hash40("common"), hash40("jump_neutral_y"));
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let pos = *PostureModule::pos(boma);
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let clatter_time = ControlModule::get_clatter_time(capture_boma, 0);
        ControlModule::set_clatter_time(capture_boma, clatter_time-1.0, 0);
        if clatter_time <= 0.0 {
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        }
    }
    if x_stick < -walk_stick_x {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN.into(), false.into());
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP.into(), false.into());
        return 0.into();
    }
    if x_stick < walk_stick_x && x_stick > -walk_stick_x {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT.into(), false.into());
    }
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
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_walk_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_walk_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let object_id = WorkModule::get_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if ![
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, 
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW
    ].contains(&status_kind) {
        if CatchModule::is_catch(boma) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(boma);
                PostureModule::set_scale(capture_boma, 1.0, false);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(boma, true);
            CatchModule::catch_cut(boma, false, false);
        }
        ArticleModule::remove_exist_object_id(boma, object_id as u32);
        ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_catch_walk_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let object_id = WorkModule::get_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if ![
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, 
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW
    ].contains(&status_kind) {
        if CatchModule::is_catch(boma) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(boma);
                PostureModule::set_scale(capture_boma, 1.0, false);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(boma, true);
            CatchModule::catch_cut(boma, false, false);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        ArticleModule::remove_exist_object_id(boma, object_id as u32);
        ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, luigi_special_lw_catch_walk_exit_status)
    .install()
    ;
}