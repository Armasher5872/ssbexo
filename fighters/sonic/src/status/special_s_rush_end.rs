use super::*;

unsafe extern "C" fn sonic_special_s_rush_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.09);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.13);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_rush_end"), L2CValue::Hash40s("special_air_s_rush_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_rush_end_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boost_value = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_rush_end"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_rush_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if boost_value > 33.0 {
            WorkModule::sub_float(fighter.module_accessor, 33.0, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_main_status)
    .status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_exec_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_end_status)
    .status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END, sonic_special_s_rush_end_exit_status)
    .install()
    ;
}