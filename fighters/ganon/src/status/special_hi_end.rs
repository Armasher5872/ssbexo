use super::*;

unsafe extern "C" fn ganon_special_hi_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(boma);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    VisibilityModule::set_whole(boma, true);
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_ignore_boss(boma, false);
    JostleModule::set_status(boma, true);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_end"), L2CValue::Hash40s("special_air_hi_end"), false.into());
    }
    else {
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::set_float(boma, 30.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("special_air_hi_end_charged"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
                WorkModule::set_float(boma, 30.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
            else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_hi_end"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_AIR {
            if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, ganon_special_hi_end_exit_status)
    .install()
    ;
}