use super::*;

unsafe extern "C" fn lucario_side_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn lucario_side_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    0.into()
}

unsafe extern "C" fn lucario_side_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_hi"), L2CValue::Hash40s("special_air_s_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_side_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_side_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_hi"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_hi"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }   
    }
    if situation_kind == *SITUATION_KIND_AIR && current_frame == 23.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_side_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_side_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_LW_INPUT);
    0.into()
}

unsafe extern "C" fn lucario_side_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_LW_INPUT);
    0.into()
}

pub fn install() {
    Agent::new("lucario")
    .status(Pre, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_pre_status)
    .status(Init, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_init_status)
    .status(Main, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_main_status)
    .status(Exec, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_exec_status)
    .status(End, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_end_status)
    .status(Exit, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, lucario_side_special_hi_exit_status)
    .install()
    ;
}