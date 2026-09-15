use super::*;

unsafe extern "C" fn wolf_special_s_rush_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_rush_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    0.into()
}

unsafe extern "C" fn wolf_special_s_rush_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let motion_speed_mul = WorkModule::get_float(boma, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(boma, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(friction_off, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_speed_mul);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_special_s_rush_main_loop as *const () as _))
}

unsafe extern "C" fn wolf_special_s_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let motion_speed_mul = WorkModule::get_float(boma, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    let rush_degree = WorkModule::get_float(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
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
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        } 
        else if StatusModule::is_situation_changed(boma) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), true.into());
            if situation_kind == *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_speed_mul);
                sv_kinetic_energy!(friction_off, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            } 
            else {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                GroundModule::select_cliff_hangdata(boma, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
            }
        }
    }
    PostureModule::set_rot(boma, &Vector3f{x: -rush_degree, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_rush_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wolf_special_s_rush_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    PostureModule::set_rot(boma, &Vector3f::zero(), 0);
    0.into()
}

unsafe extern "C" fn wolf_special_s_rush_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_pre_status)
    .status(Init, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_init_status)
    .status(Main, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_main_status)
    .status(Exec, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_exec_status)
    .status(End, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_end_status)
    .status(Exit, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, wolf_special_s_rush_exit_status)
    .install()
    ;
}