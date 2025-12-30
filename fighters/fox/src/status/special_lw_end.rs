use super::*;

unsafe extern "C" fn fox_special_lw_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_7100018450(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(fox_special_lw_end_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100018450(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut ground_motion = "special_lw_end";
    let mut air_motion = "special_air_lw_end";
    if lr == -1.0 {
        ground_motion = "special_lw_end_l";
        air_motion = "special_air_lw_end_l";
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_motion), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(air_motion), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);    
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(ground_motion), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);    
        }
    }
}

unsafe extern "C" fn fox_special_lw_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let mut change_motion = false;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)  {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                change_motion = true;
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                change_motion = true;
            }
        }
    }
    if change_motion {
        fun_7100018450(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT) {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            return 0.into();
        }
        if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn fox_special_lw_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    0.into()
}

unsafe extern "C" fn fox_special_lw_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    0.into()
}

pub fn install() {
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, fox_special_lw_end_main_status)
    .status(End, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, fox_special_lw_end_end_status)
    .status(Exit, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, fox_special_lw_end_exit_status)
    .install()
    ;
}