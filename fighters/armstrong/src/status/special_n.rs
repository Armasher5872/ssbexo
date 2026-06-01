use super::*;

unsafe extern "C" fn armstrong_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n"), L2CValue::Hash40s("special_air_n"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_n_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_n_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        if StatusModule::is_situation_changed(boma) {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if MotionModule::is_end(boma) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_CHARGE.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_ATTACK.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if [*FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_ATTACK].contains(&status_kind) {
        WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if [*FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_ATTACK].contains(&status_kind) {
        WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, armstrong_special_n_exit_status)
    .install()
    ;
}