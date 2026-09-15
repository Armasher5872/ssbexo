use super::*;

unsafe extern "C" fn wolf_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let illusion_accel_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_accel_x"));
    let illusion_start_x_mul = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_start_x_mul"));
    let illusion_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    let illusion_start_speed = get_sum_speed_x*illusion_start_x_mul;
    WorkModule::set_int(boma, illusion_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    if situation_kind != *SITUATION_KIND_AIR {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_start_speed, 0.0);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 6, 0.0, illusion_start_speed, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_accel_x, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if illusion_stop_y_frame != 0 {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
    0.into()
}

unsafe extern "C" fn wolf_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let revert_angle_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("revert_angle_frame"));
    let max_rush_degree = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("max_rush_degree"));
    let min_stick_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("min_stick_y"));
    let illusion_rush_speed_mul = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_rush_speed_mul"));
    let illusion_rush_speed_mul_power_up = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_rush_speed_mul_power_up"));
    WorkModule::set_int(boma, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    WorkModule::set_int(boma, revert_angle_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    WorkModule::set_int(boma, 0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    WorkModule::set_float(boma, max_rush_degree, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    WorkModule::set_float(boma, min_stick_y, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    PostureModule::set_stick_lr(boma, 0.0);
    PostureModule::update_rot_y_lr(boma);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        //KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(boma, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(boma, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_float(boma, illusion_rush_speed_mul*illusion_rush_speed_mul_power_up, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    if !StopModule::is_stop(boma) {
        wolf_special_s_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(wolf_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn wolf_special_s_substatus(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if bool_check.get_bool() {
        WorkModule::dec_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    0.into()
}

unsafe extern "C" fn wolf_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let stick_y = ControlModule::get_stick_y(boma);
    let min_stick = WorkModule::get_float(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    let max_rush_degree = WorkModule::get_float(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) && (MotionModule::is_end(boma) || StatusModule::is_situation_changed(boma)) {
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH.into(), false.into());
            return 0.into();
        } 
        else if StatusModule::is_situation_changed(boma) {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            } 
            else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), true.into());
        }
    }
    WorkModule::set_float(boma, (stick_y.signum()*max_rush_degree*f32::max(stick_y.abs()-min_stick, 0.0)/(1.0-min_stick)).clamp(-20.0, 25.0), *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
    0.into()
}

unsafe extern "C" fn wolf_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let stop_y_frame = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    let illusion_accel_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_accel_y"));
    let illusion_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    if situation_kind == *SITUATION_KIND_AIR {
        if stop_y_frame == 0 && illusion_stop_y_frame != 0 {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_accel_y);
        }
    }
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wolf_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, wolf_special_s_exit_status)
    .install()
    ;
}