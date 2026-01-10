use super::*;

//Side Special Slide Pre Status
unsafe extern "C" fn wario_special_s_slide_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32, 0);
    0.into()
}

//Side Special Slide Init Status
unsafe extern "C" fn wario_special_s_slide_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_speed*1.25);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, ground_brake*0.25);
    0.into()
}

//Side Special Slide Main Status
unsafe extern "C" fn wario_special_s_slide_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_slide"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_s_slide_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_s_slide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
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
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

//Side Special Slide Exec Status
unsafe extern "C" fn wario_special_s_slide_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Side Special Slide End Status
unsafe extern "C" fn wario_special_s_slide_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    0.into()
}

//Side Special Slide Exit Status
unsafe extern "C" fn wario_special_s_slide_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE, wario_special_s_slide_exit_status)
    .install()
    ;
}