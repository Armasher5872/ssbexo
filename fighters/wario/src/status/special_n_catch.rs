use super::*;

//Neutral Special Catch Pre Status
unsafe extern "C" fn wario_special_n_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Catch Init Status
unsafe extern "C" fn wario_special_n_catch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    HitModule::set_invincible_frame_global(boma, 12, false, 0);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, true);
    sv_module_access::capture(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let shouldered_frame_add= WorkModule::get_param_float(capture_boma, hash40("common"), hash40("shouldered_frame_add"));
        let damage = DamageModule::damage(capture_boma, 0);
        let total_time = damage+(shouldered_frame_add*1.3);
        ControlModule::start_clatter(capture_boma, total_time, 0.0, 9.0, 127, 0, false, false);
    }
    0.into()
}

//Neutral Special Catch Main Status
unsafe extern "C" fn wario_special_n_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_catch"), L2CValue::Hash40s("special_air_n_catch"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_catch_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_catch"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_catch"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Neutral Special Catch Exec Status
unsafe extern "C" fn wario_special_n_catch_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch End Status
unsafe extern "C" fn wario_special_n_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_end(fighter, false);
    0.into()
}

//Neutral Special Catch Exit Status
unsafe extern "C" fn wario_special_n_catch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_exit(fighter, false);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, wario_special_n_catch_exit_status)
    .install()
    ;
}