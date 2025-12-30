use super::*;

unsafe extern "C" fn fox_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let reflector_air_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_air_stop_y_frame"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, reflector_air_stop_y_frame, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    fun_7100020170(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(fox_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100020170(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut ground_motion = "special_lw_start";
    let mut air_motion = "special_air_lw_start";
    if lr == -1.0 {
        ground_motion = "special_lw_start_l";
        air_motion = "special_air_lw_start_l";
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
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
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(ground_motion), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        }
    }
}

unsafe extern "C" fn fox_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let mut change_motion = false;
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
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
        fun_7100020170(fighter);
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

unsafe extern "C" fn fox_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    }
    0.into()
}

unsafe extern "C" fn fox_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    }
    0.into()
}

pub fn install() {
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, fox_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, fox_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, fox_special_lw_exit_status)
    .install()
    ;
}