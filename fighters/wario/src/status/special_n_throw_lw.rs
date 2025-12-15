use super::*;

//Neutral Special Down Throw Pre Status
unsafe extern "C" fn wario_special_n_throw_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Down Throw Init Status
unsafe extern "C" fn wario_special_n_throw_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_speed);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.01);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let shouldered_frame_add= WorkModule::get_param_float(capture_boma, hash40("common"), hash40("shouldered_frame_add"));
        let shouldered_frame_mul = WorkModule::get_param_float(capture_boma, hash40("common"), hash40("shouldered_frame_mul"));
        let damage = DamageModule::damage(capture_boma, 0);
        let get_clatter_time = ControlModule::get_clatter_time(capture_boma, 0);
        let total_time = (damage*shouldered_frame_mul)+shouldered_frame_add+get_clatter_time;
        ControlModule::start_clatter(capture_boma, total_time, 0.0, 8.0, 127, 0, false, false);
    }
    0.into()
}

//Neutral Special Down Throw Main Status
unsafe extern "C" fn wario_special_n_throw_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter, "bitten_wario_start", 50.0, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_throw_lw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_throw_lw_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_throw_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let pos = *PostureModule::pos(fighter.module_accessor);
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let clatter_time = ControlModule::get_clatter_time(capture_boma, 0);
        ControlModule::set_clatter_time(capture_boma, clatter_time-1.0, 0);
        if clatter_time <= 0.0 {
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Neutral Special Down Throw Exec Status
unsafe extern "C" fn wario_special_n_throw_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Down Throw End Status
unsafe extern "C" fn wario_special_n_throw_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if CatchModule::is_catch(fighter.module_accessor) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(fighter.module_accessor);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(fighter.module_accessor, true);
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_wario_final01"), 0);
    }
    0.into()
}

//Neutral Special Down Throw Exit Status
unsafe extern "C" fn wario_special_n_throw_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let pos = *PostureModule::pos(fighter.module_accessor);
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
            sv_module_access::_catch(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_wario_final01"), 0);
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_exit_status)
    .install()
    ;
}