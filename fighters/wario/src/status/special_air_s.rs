use super::*;

//Aerial Side Special Pre Status
unsafe extern "C" fn wario_special_air_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Aerial Side Special Init Status
unsafe extern "C" fn wario_special_air_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    0.into()
}

//Aerial Side Special Main Status
unsafe extern "C" fn wario_special_air_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_air_s_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_air_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        if current_frame < 15.0 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH.into(), false.into());
        }
        else if current_frame >= 15.0 && current_frame < 23.0 {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if current_frame >= 23.0 && current_frame < 40.0 {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), current_frame-23.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        if lr == -1.0 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END.into(), false.into());
        }
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        if lr == 1.0 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END.into(), false.into());
        }
    } 
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Aerial Side Special Exec Status
unsafe extern "C" fn wario_special_air_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Aerial Side Special End Status
unsafe extern "C" fn wario_special_air_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

//Aerial Side Special Exit Status
unsafe extern "C" fn wario_special_air_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, wario_special_air_s_exit_status)
    .install()
    ;
}