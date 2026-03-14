use super::*;

unsafe extern "C" fn armstrong_special_s_run_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_run_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.2);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_run_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_run"), L2CValue::Hash40s("special_air_s_run"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_s_run_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_s_run_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let charge_frames = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0*lr, 0.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_run"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_run"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_SPECIAL_S_RUN_TIME) > 10+charge_frames {
        fighter.change_status(FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_run"), L2CValue::Hash40s("special_air_s_run"), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_s_run_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_SPECIAL_S_RUN_TIME);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_run_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH].contains(&status_kind) {
        armstrong_clear_charge(fighter.module_accessor);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_SPECIAL_S_RUN_TIME);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_run_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH].contains(&status_kind) {
        armstrong_clear_charge(fighter.module_accessor);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_SPECIAL_S_RUN_TIME);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_pre_status)
    .status(Init, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_init_status)
    .status(Main, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_main_status)
    .status(Exec, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_exec_status)
    .status(End, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_end_status)
    .status(Exit, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, armstrong_special_s_run_exit_status)
    .install()
    ;
}