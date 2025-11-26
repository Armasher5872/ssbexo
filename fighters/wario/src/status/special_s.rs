use super::*;

//Grounded Side Special Pre Status
unsafe extern "C" fn wario_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Grounded Side Special Init Status
unsafe extern "C" fn wario_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_all(fighter.module_accessor);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

//Grounded Side Special Main Status
unsafe extern "C" fn wario_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S.into(), false.into());
        return 0.into();
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_s_main_loop as *const () as _))
    }
}

unsafe extern "C" fn wario_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP.into(), false.into());
        return 0.into();
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        if lr == -1.0 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END.into(), false.into());
            return 0.into();
        }
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        if lr == 1.0 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END.into(), false.into());
            return 0.into();
        }
    } 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Grounded Side Special Exec Status
unsafe extern "C" fn wario_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Grounded Side Special End Status
unsafe extern "C" fn wario_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    }
    0.into()
}

//Grounded Side Special Exit Status
unsafe extern "C" fn wario_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, wario_special_s_exit_status)
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, wario_special_s_exit_status)
    .install()
    ;
}