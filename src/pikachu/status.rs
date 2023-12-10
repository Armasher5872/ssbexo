use super::*;

/*   IRON TAIL START STATUS SCRIPTS   */

unsafe extern "C" fn pikachu_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SKULL BASH HOLD STATUS SCRIPTS   */
//0'd out to prevent any Skull Bash logic from leaking into Iron Tails logic

unsafe extern "C" fn pikachu_special_s_hold_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SKULL BASH WEAK STATUS SCRIPTS   */
//0'd out to prevent any Skull Bash logic from leaking into Iron Tails logic

unsafe extern "C" fn pikachu_special_s_weak_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_weak_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_weak_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_weak_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_weak_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_weak_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   IRON TAIL STATUS SCRIPTS   */

unsafe extern "C" fn pikachu_special_s_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn pikachu_special_s_attack_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_HIT);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(SITUATION_KIND_AIR.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_attack_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_attack_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn pikachu_special_s_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SKULL BASH END STATUS SCRIPTS   */
//0'd out to prevent any Skull Bash logic from leaking into Iron Tails logic

unsafe extern "C" fn pikachu_special_s_end_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_end_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("pikachu")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_exit_status)
    .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_pre_status)
    .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_init_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_main_status)
    .status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_exec_status)
    .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_end_status)
    .status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_exit_status)
    .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_pre_status)
    .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_init_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_main_status)
    .status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_exec_status)
    .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_end_status)
    .status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, pikachu_special_s_weak_exit_status)
    .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_pre_status)
    .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_init_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_main_status)
    .status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_exec_status)
    .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_end_status)
    .status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, pikachu_special_s_attack_exit_status)
    .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_pre_status)
    .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_init_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_main_status)
    .status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_exec_status)
    .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_end_status)
    .status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pikachu_special_s_end_exit_status)
    .install()
    ;
}