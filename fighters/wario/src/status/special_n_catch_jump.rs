use super::*;

//Neutral Special Catch Jump Pre Status
unsafe extern "C" fn wario_special_n_catch_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Catch Jump Init Status
unsafe extern "C" fn wario_special_n_catch_jump_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch Jump Main Status
unsafe extern "C" fn wario_special_n_catch_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    ControlModule::reset_flick_x(boma);
    ControlModule::reset_flick_sub_x(boma);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xFE));
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    MotionModule::change_motion(boma, Hash40::new("special_n_catch_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_catch_jump_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_catch_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
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
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Neutral Special Catch Jump Exec Status
unsafe extern "C" fn wario_special_n_catch_jump_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch Jump End Status
unsafe extern "C" fn wario_special_n_catch_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if CatchModule::is_catch(boma) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(boma);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(boma, true);
            CatchModule::catch_cut(boma, false, false);
        }
    }
    0.into()
}

//Neutral Special Catch Jump Exit Status
unsafe extern "C" fn wario_special_n_catch_jump_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(boma);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
            sv_module_access::_catch(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_exit_status)
    .install()
    ;
}