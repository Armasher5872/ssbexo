use super::*;

unsafe extern "C" fn sonic_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_all(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if stick_y > 0.7 {
        WorkModule::set_int(fighter.module_accessor, 20, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    }
    else if stick_y < -0.7 {
        WorkModule::set_int(fighter.module_accessor, -20, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_rush_shock"), true, true);
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_rush_shock"), true, true);
    }
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, sonic_special_s_exit_status)
    .install()
    ;
}