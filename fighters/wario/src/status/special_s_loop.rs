use super::*;

//Side Special Loop Pre Status
unsafe extern "C" fn wario_special_s_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Side Special Loop Init Status
unsafe extern "C" fn wario_special_s_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
    0.into()
}

//Side Special Loop Main Status
unsafe extern "C" fn wario_special_s_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_s_loop_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_s_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT.into(), false.into());
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP.into(), false.into());
        return 0.into();
    }
    if timer >= 35 {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END.into(), false.into());
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
    if timer < 15 {
        if stick_y < -0.66 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SLIDE.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Side Special Loop Exec Status
unsafe extern "C" fn wario_special_s_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Side Special Loop End Status
unsafe extern "C" fn wario_special_s_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    }
    0.into()
}

//Side Special Loop Exit Status
unsafe extern "C" fn wario_special_s_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, wario_special_s_loop_exit_status)
    .install()
    ;
}