use super::*;

unsafe extern "C" fn wolf_special_s_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let illusion_end_air_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
    let illusion_end_air_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_speed_x"));
    let illusion_end_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_brake_x"));
    let illusion_end_c3_speed_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_c3_speed_y"));
    let illusion_end_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_speed_x"));
    let illusion_end_air_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_end_air_stop_y_frame"));
    WorkModule::set_int(boma, illusion_end_air_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    if situation_kind != *SITUATION_KIND_AIR {
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_brake_x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_speed_x*lr, 0.0);
    }
    else {
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_speed_x*lr, 0.0);
    }
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 10.0, 10.0);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, illusion_end_c3_speed_y);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 10.0);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(boma, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(boma, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    if !StopModule::is_stop(boma) {
        wolf_special_s_end_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(wolf_special_s_end_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn wolf_special_s_end_substatus(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if bool_check.get_bool() {
        WorkModule::dec_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        WorkModule::inc_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    }
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let rush_degree = WorkModule::get_float(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
    let revert_angle_count = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    let revert_angle_frame = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    let mut angle = 0.0;
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
    if !StatusModule::is_changing(boma) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, boma);
        if MotionModule::is_end(boma) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
            return 0.into();
        } 
        else if StatusModule::is_situation_changed(boma) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_end"), L2CValue::Hash40s("special_air_s_end"), true.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 0.into();
            } 
            else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
        }
    }
    if revert_angle_count < revert_angle_frame && 0 < revert_angle_frame {
        angle = rush_degree-(rush_degree*revert_angle_count as f32 / revert_angle_frame as f32);
    }
    PostureModule::set_rot(boma, &Vector3f{x: -angle, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let stop_y_frame = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    let illusion_end_air_accel_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_accel_y"));
    let illusion_end_air_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
    if situation_kind == *SITUATION_KIND_AIR {
        if illusion_end_air_brake_x != {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP); sv_kinetic_energy::get_brake_x(fighter.lua_state_agent)} {
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
        }
        if stop_y_frame == 0 {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_end_air_accel_y);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    PostureModule::set_rot(boma, &Vector3f::zero(), 0);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    }
    0.into()
}

unsafe extern "C" fn wolf_special_s_end_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_pre_status)
    .status(Init, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_init_status)
    .status(Main, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_main_status)
    .status(Exec, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_exec_status)
    .status(End, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_end_status)
    .status(Exit, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, wolf_special_s_end_exit_status)
    .install()
    ;
}