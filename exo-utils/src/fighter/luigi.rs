use super::*;

pub unsafe extern "C" fn luigi_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_BATABATA);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
}

pub unsafe extern "C" fn luigi_unlink_end(boma: *mut BattleObjectModuleAccessor) {
    if CatchModule::is_catch(boma) {
        let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let capture_status = StatusModule::status_kind(capture_boma);
            let pos = *PostureModule::pos(boma);
            if capture_status != *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN {
                StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, false);
            }
            PostureModule::set_scale(capture_boma, 1.0, false);
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        }
        CatchModule::set_send_cut_event(boma, true);
        CatchModule::catch_cut(boma, false, false);
    }
}

pub unsafe extern "C" fn luigi_unlink_exit(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
        let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let capture_status = StatusModule::status_kind(capture_boma);
            let pos = *PostureModule::pos(boma);
            if capture_status != *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN {
                StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, false);
            }
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

pub unsafe extern "C" fn luigi_special_lw_end(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let object_id = WorkModule::get_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if ![
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW
    ].contains(&status_kind) {
        luigi_unlink_end(boma);
        ArticleModule::remove_exist_object_id(boma, object_id as u32);
        ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
}

pub unsafe extern "C" fn luigi_special_lw_exit(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let object_id = WorkModule::get_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if ![
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WAIT, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_TURN, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_WALK, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_THROW
    ].contains(&status_kind) {
        luigi_unlink_end(boma);
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        ArticleModule::remove_exist_object_id(boma, object_id as u32);
        ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
}