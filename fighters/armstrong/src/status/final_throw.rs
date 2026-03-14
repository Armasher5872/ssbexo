use super::*;

unsafe extern "C" fn armstrong_final_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_final_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, true);
    sv_module_access::capture(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        VisibilityModule::set_whole(capture_boma, false);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn armstrong_final_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter, "down_damage", 0.0, 0.0);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("final_throw"), L2CValue::Hash40s("final_air_throw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_final_throw_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_final_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_throw"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_air_throw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FINAL_THROW) {
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            VisibilityModule::set_whole(capture_boma, true);
            AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FINAL_THROW);
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

unsafe extern "C" fn armstrong_final_throw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_final_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FINAL_THROW);
    0.into()
}

unsafe extern "C" fn armstrong_final_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_FINAL_THROW);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_pre_status)
    .status(Init, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_init_status)
    .status(Main, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_main_status)
    .status(Exec, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_exec_status)
    .status(End, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_end_status)
    .status(Exit, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, armstrong_final_throw_exit_status)
    .install()
    ;
}